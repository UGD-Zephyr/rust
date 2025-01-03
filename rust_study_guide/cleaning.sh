#!/usr/bin/bash

# Programmer:           Per Stoor
# Date:                 2025-01-04
# Last changed:         2025-01-04
# Type of script:       

for loop_counter in {1..7}
do 
    rm -r ./grok_study_guide/grok_$loop_counter/.git
done

for loop_counter in {1..7}
do 
    rm ./grok_study_guide/grok_$loop_counter/.gitignore
done
