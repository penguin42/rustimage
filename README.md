Toy image transform

This is a toy project of mine to learn Rust that implements an image
transform.
The idea is that you take a macro image with the object in question surrounded
by a black 2:1 ratio rectangle printed on white paper.  The program finds the
rectangle (which may be bowed due to lense distortion and rotated) and
produces a nicely straightened result image.

TODO:
   Use interpolation to produce each pixel from its neighbours depending
     on the fractional location.
   Come up with a more convincing control point for the middle bezier.
   Take options for output size
   Take parameter for output name

