# Start 

This project was originally started as a suggestion by ChatGPT.

The purpose was to rebuild malloc / realloc / free from C.

## Review

Ultimately its a bit of a silly idea. Fundamentally Rust has no need for any of these methods because the model 
that it uses to interface with memory is entirely different from that of C. Where C requires you to handle memory directly
Rust will do it automatically.

The one model fundamentally is about saying "I need 50 megabytes for this object" and the other "give me an object". 

## Evolution

The project eventually evolved into something which let me play around with assigning any object with an implementation
of a type into a vector. 

Simple but it was fun