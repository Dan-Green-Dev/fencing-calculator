# Fencing Competiton Calculator

This is my attempt to practice file handling, and research into how a fencing competition is ran and how the ranking system works
It also makes it easier to check quality of competition ahead of time, to evaluate if it is worth attending

## Use Case

This is designed for UK competitions, where the user can input the entries from bf.sports80.com, run it through the app and then get useful data.

### input.csv

There is an input.csv file, this is where the sports80 entries should be pasted.
They go in the format:

Fencer name


Club<br>
Fencer name

The 2 line gap between Fencer name and club is there because that is how a direct paste from sports80 will do it, so it is important to ensure that is kept.

### src/*.html

The mixed foil and womens national ranking files are stored in here, as well as men's foil from 1st december 2024. If any more are needed for your use case, add them into the /src folder.

I plan to improve functionality so other categories can be handled more smoothly, however at this moment, lines 49 and 50 will have to be commented out, and line 101 will have to be changed to match the ranking file being used.

## Future functionality

- Easy switching between categories
- A look into including the results of BUCS individuals
- Rating system integration
- GUI output, or at least more readable output
