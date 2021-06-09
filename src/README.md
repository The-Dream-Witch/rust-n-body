<h6>
Mallory S. Hawke</br>
CS410P, Spring 2021</br>
Homework 4
</h6>
<div align = "center">
<h1>N-Body-Sim</h1>
</div>

<H3>Intro</H3>
Hey there! Welcome to my semi-graphical N-Body simulator. 
</br></br>
Originally, the intent with this project (my final project for Bart Massey's Introduction to Rust class) was to develop a solution to the N-Body problem with the following characterstics:

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

Still, I managed to hit enough of them that I'm proud of what I managed to produce in such a short time. If nothing else, this was an incredible learning experience, and I fully intend to continue my work on this far past the assignments due date in an attempt to reach those goals I hadn't managed to hit.

<h3>So, what does it actually do?</h3>
Essentially, as the crate stands now, I have managed to:</br></br>

* Develop a naive solution to the n-body problem.
* Develop a barnes-hut / OctTree solution to the n-body problem
* Render the results of those solutions in pseudo-3D* using the <a href="https://www.piston.rs/">Piston</a> library


Currently, the reasonable limit for n using my naive solution is `n = ~500`, while the barnes-hut implementation begins to chug pretty heavily around `n = 2000~3000`; these values could likey be significantly increased by using a proper vector / matrix multiplication library, CPU multithreading, or GPU-based calculations; I intend to pursue all such options in the future.

(Note: 'Pseudo-3D' simply means 2D rendering with 3D calculations, utilizing the color and size of a given body to indicate depth / distance)

<h3>How does it do it?</h3>

The main body of the program contains the rendering

<h3>How do we know it works/doesn't break?</h3>

