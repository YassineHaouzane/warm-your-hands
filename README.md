# Warm your hands

This program heats up your cpu to warm your hands.

This is better used on a laptop.
This program will run expensive operation until the cpu's temperature reaches the `[MAX_TEMP]` option.

# Running

`warm-your-hands [-m MAX_TEMP] [-c CACHE_SIZE]`

You can specify the max temperature your cpu should reach with the -m option. if not specified it will default to 60.
By specifying the CACHE_SIZE you can specify L3 cache_size this is needed to excute expensive operations.
