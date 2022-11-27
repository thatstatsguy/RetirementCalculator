# RetirementCalculator

## TLDR
This is a fun application which gives users a graphical illustration of how their choices for retirement contributions affect their lump sums for retirement retirement.

## More info

## Problem Statement
We all want to retire comfortably, but it's sometimes difficult to visualise how contributions you make today affect your retirement in 40 years from now.

This application allows you to set your contributions for X years and arbitrarily change it for various intervals.

Why bother with something like this? When I started working, I always wondered
> I currently contribute X% to my retirement
> savings. 
>
> If it was possible to contribute X  + 10% 
> for 5 years and then X% for the rest of my
> career, how much would this affect my final
> retirement contributions?


This application aims to address the above question in an interactive way.

## The Technical Stuff
It would be fairly easy to bring together the project in a single application, but what's the fun in that. The following topics will be touched while building this application. 

1. Domain Driven Design 
    - the aim is to revisit domain events prior to and during the building phase as an experiment in capturing all domain events and actors.
2. Rust schenanigans
    - The server doing the calculations will be in rust (which I know nothing about)
3. Microservice communication via queues
    - because I want to play around with queues 
4. Some kind of visualisation library
5. Docker compose to tie the application together.

## Resources

Useful guides for referencing in future

### Rust
[Setting up Rust in Windows](https://www.twelve21.io/getting-started-with-rust-on-windows-and-visual-studio-code/#:~:text=Interestingly%20enough%2C%20the%20Rust%20compiler%20requires%20the%20Microsoft,the%20Visual%20Studio%20Build%20Tools%202019%20version%20here.)