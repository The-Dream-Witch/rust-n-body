<h6>
Mallory S. Hawke</br>
CS410P, Spring 2021</br>
Homework 4
</h6>
<div align = "center">
<h1>N-Body-Sim</h1>
</div>

<H3>Intro</H3>
Hey there! Welcome to my final project for Bart Massey's Introduction to Rust class.
</br></br>
This is a very bare-bones code base which focuses on implementing methods to calculate newtonian gravitation interactions between some number of bodies, n, and then display the results of those calculations graphically.

<h3>So, what does it actually do?</h3>

At a macro level, the crate will do the following:
* Allow the user to calculate newtonian gravitational interactions between some specified number of bodies, n, using either:
    - The bruteforce algorithm
    - The Barnes-Hut / OctTree algorithm
* Display the results of those operations, graphically, in pseudo-3D (2D rendering with 3D calculations, utilizing colour and size of particles to indicate depth / distance)

Currently, the reasonable limit for n using my naive solution is `n = ~500`, while the barnes-hut implementation begins to chug pretty heavily around `n = 2000~3000`; these values could likey be significantly increased by using a proper vector / matrix multiplication library, CPU multithreading, or GPU-based calculations.
<h3>What Went Wrong?</h3>
Originally, the intent with this project was to develop a solution to the N-Body problem with the following characterstics:

* Able to support n > 10,000
* Rendered in 3D
* Could create large interacting bodies such as galaxies
* Utilizes GPU for calculations
* Utilizes Barnes-Hut algorithm

The inspiration for this was YouTube video of an <a href="https://www.youtube.com/watch?v=x62gOfZ9hCw">ASCII Simulation of Colliding Galaxies</a>, the codebase for which can be found at <a href="https://github.com/DinoZ1729/Galaxy">DinoZ1729's Github</a>.

However, going into project this I had little-to-no experience:
* Working with graphical libraries, either 2D or 3D.
* Calculating gravitational interactions between bodies.
* Creating large Rust projects from whole cloth
* Performing GPU computations

So, given that I had basically no idea what I was doing, and that nearly all of the work was performed in the last week-and-a-half of the term due to other assignments taking short-term precedence, it should come as no surprise that I didn't manage to hit ***all*** of the (honestly, very hubristic) goals I'd set for myself. 

In particular, GPU calculations, true 3D, and the n = 10,000 target had to be droppped.

Still, I managed to hit enough of my goals that I'm proud of what I managed to produce in such a short time. If nothing else, this was an incredible learning experience, and I fully intend to continue my work on this far past the assignments due date in an attempt to reach those goals I hadn't managed to hit.

<h3>How do I run it?</h3>

Pretty simple, honestly. Just clone it, go to that directory, and enter on your command line:

`cargo run (x) (y)`

- The first argument, x, must be either 1 or 2; 1 indicates that you want to use the naive / brute force method for calculating interactions between bodies, and 2 indicates that you want to use the barnes-hut / oct tree. 
- The second argument, y, must be a positive integer less than 2^32. While it is otherwise unbounded, it may be useful to note that numbers larger than ~1000 for the naive

TLDR:

1) Clone the repository to your computer
2) Change your directory to the directory you cloned the repository to
3) In the command line type: cargo run (x) (y) (where x is an integer from 1 to 2, and y is a positive integer < 2^32)

<h3>How do we know it works/doesn't break?</h3>

I've managed to implement a fair number of unit tests for most things:

<h5>Vec3D</h5> Has tests which generate pseudo-random values and perform local calculations to check that the implemented ops, the sum of squares function, and the scalar (basically just returns DT / r^2) return the correct values.

<h5>Nbodies</h5> Implements tests which try to make sure that the algorithm's won't attempt to update a body if the thing it's updating against is that same body.

<h5>Parsearg and parsenum</h5>The argument parser and number parser functions implement a huge battery of tests which try to make sure that there isn't
<h5>Body</h5>
The only function really implemented for body is the bounds function, and so that is what the sole unit test here focuses on; a body is created and its position values are generated at random, but are positive. The coordinates are then increased to beyond the allowable values 

<h3>Acknowledgements and Citations</h3>
I came into this project feeling pretty overwhelmed by the scope of what I'd agreed to, especially considering I've never worked on anything grapical besides some ncurses, my practical
understanding of the physics involved was initially lacking, and the time I had to dedicate to the project was miniscule. In fact, it was only within the last week-and-a-half that I was
able to focus on it with any regularity, as my other classes kept me very much on my toes with a consistent barrage of assignments. 
</br></br>
Thus, it was necessary for me to consult other, similar, projects for inspiration:
</br></br>

- <a href = "https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started">Piston's Spinning Square Tutorial</a> was helpful in showing me how to set up and use piston.
- <a href="https://github.com/dileban/nbody-simulation/blob/master/pnbody.c">Dileban's nbody-simulation</a> provided an understanding of the basics of gravitational interactions.
- <a href="https://github.com/zongzhengli/n-body/blob/master/Source/Octree.cs">Zongzhengli's n-body</a> was absolutely instrumental in understanding the barnes-hut algorithm.