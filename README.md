# Bash `$RANDOM` Cracker

A tool to brute-force the internal seed of Bash's `$RANDOM` variable after only 2-3 15-bit samples. Able to predict all future values to break the randomness. 

For context, the `bash` shell has a dynamic variable called `$RANDOM` you can access at any time to receive a random 15-bit number:

```Shell
$ echo $RANDOM $RANDOM $RANDOM
3916 29151 6095
```

To seed this random number generator, you can set the variable directly to get the same values every time:

```Shell
$ RANDOM=1337; echo $RANDOM $RANDOM $RANDOM
24879 21848 15683
$ RANDOM=1337; echo $RANDOM $RANDOM $RANDOM
24879 21848 15683
```

There are **2 different calculations** depending on your **bash version**, which may make one seed give two different outputs. All versions *>= 5.1* will add an extra step, and to this tool are considered the "new" versions, while any lower versions are considered "old". This can be set explicitly using the `--version` (`-v`) argument in this tool, or otherwise it will simply try both. 

## Usage

<!-- TODO: asciinema recording -->

<!-- 
bash --version
echo $RANDOM $RANDOM $RANDOM
bashrand crack 664 2679 3415
echo $RANDOM $RANDOM $RANDOM
bashrand get 892556151
bashrand get 892556151 -v old
bashrand get 892556151 -v old -s 6
bashrand get 892556151 -v old -s 6 -n 3
echo $RANDOM $RANDOM $RANDOM
exit
 -->

#### Help

```Shell
$ bashrand --help
Bash $RANDOM Cracker

Usage: bashrand [OPTIONS] <COMMAND>

Commands:
  crack  Provide random numbers to brute-force the seed
  get    Get random numbers from a seed
  help   Print this message or the help of the given subcommand(s)

Options:
  -v, --version <VERSION>
          Which bash version to use for generation (check with `bash --version`)

          [default: both]

          Possible values:
          - old:  Bash versions 5.0 and older
          - new:  Bash versions 5.1 and newer
          - both: Try both old and new versions if unsure

  -n, --number <NUMBER>
          Number of values to generate

          [default: 10]
```

```Shell
$ bashrand crack --help
Provide random numbers to brute-force the seed

Usage: bashrand crack [OPTIONS] [NUMBERS] [NUMBERS]...

Arguments:
  [NUMBERS] [NUMBERS]...
          2-3 $RANDOM numbers as input for brute-forcing the seed

          2 => multiple possible seeds, 3 => single seed
```

```Shell
$ bashrand get --help
Get random numbers from a seed

Usage: bashrand get [OPTIONS] <SEED>

Arguments:
  <SEED>
          Seed to use for generating random numbers
```

## Installation



## Reverse Engineering

To implement the `$RANDOM` algorithm, the first requirement is understanding the algorithm. Luckily Bash is open-source meaning all the clear and documented code is available. I used [this repository](https://github.com/bminor/bash) to look for anything related to the generation of this variable, and found the definition here:

https://github.com/bminor/bash/blob/ec8113b9861375e4e17b3307372569d429dec814/variables.c#L1914

It assigns two functions to the variable: [`get_random()`](https://github.com/bminor/bash/blob/ec8113b9861375e4e17b3307372569d429dec814/variables.c#L1443-L1450) and [`assign_random`](https://github.com/bminor/bash/blob/ec8113b9861375e4e17b3307372569d429dec814/variables.c#L1401-L1420). The first is when you access the variable like `echo $RANDOM`, and the second is for when you assign a value yourself to the variable, like `RANDOM=1337`. 

`get_random()` is the most interesting as we want to predict its output. It calls the a [`get_random_number()`](https://github.com/bminor/bash/blob/ec8113b9861375e4e17b3307372569d429dec814/variables.c#L1422C1-L1440) function which itself calls [`brand()`](https://github.com/bminor/bash/blob/ec8113b9861375e4e17b3307372569d429dec814/lib/sh/random.c#L98C1-L112) inside the `/lib/sh/random.c` file. Here it starts to get interesting:

https://github.com/bminor/bash/blob/ec8113b9861375e4e17b3307372569d429dec814/lib/sh/random.c#L98C1-L112

First, notice the `BASH_RAND_MAX` variable that is a 15-bit mask over the output. Also the `shell_compatibility_level` is the bash version, meaning if it is greater than version 50 (5.0) it will use a slightly different calculation. In both cases however it first gets a random number from [`intrand32()`](https://github.com/bminor/bash/blob/ec8113b9861375e4e17b3307372569d429dec814/lib/sh/random.c#L55-L84), and that already contains the core of the algorithm!

https://github.com/bminor/bash/blob/ec8113b9861375e4e17b3307372569d429dec814/lib/sh/random.c#L73-L83

These are some simple calculations that we can recreate in any programming language. Importantly, it uses a `last` variable as its only argument in the calculation, which is given by `rseed = intrand32(rseed)` in the calling function. This means there is an internal seed that is iterated every time this function is called. If we can sync up with this seed, we will be able to predict any future values by copying the algorithm. 

The initial seed value is [complicated](https://github.com/bminor/bash/blob/ec8113b9861375e4e17b3307372569d429dec814/lib/sh/random.c#L87-L96), and is calculated with a lot unpredictable data. If you remember it was also possible to *set* the seed, using [`assign_random()`](https://github.com/bminor/bash/blob/ec8113b9861375e4e17b3307372569d429dec814/variables.c#L1401-L1420). Looking at this function, it takes the value we set it to, and passes it to [`sbrand()`](https://github.com/bminor/bash/blob/ec8113b9861375e4e17b3307372569d429dec814/lib/sh/random.c#L115-L121), a very simple function that simply sets the seed directly to the provided value:

https://github.com/bminor/bash/blob/ec8113b9861375e4e17b3307372569d429dec814/lib/sh/random.c#L115-L121

So in theory if the seed was set manually, we could now simply try many seeds until we find one that matches the output. But what about seeds that aren't set manually? This case happens a lot more often. Luckily, the internal seed is an integer of only **32 bits**, easily brute-forcable with such a fast algorithm. After some testing we can find the search space is actually only 30 bits for the newer bash versions, and 31 bits for old bash versions. 

This program implements this brute-force method to search through the whole space in a few seconds, and shows the found seeds together with future values it predicts. 