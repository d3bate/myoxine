# Myoxine

## Preamble

Myoxine is a (planned) blazing-fast GraphQL runtime to design web applications which run using Rust
compiled to WebAssembly. Myoxine relies on Yew, a framework for building web applications in Rust 
analagous to React, Vue or Angular, to display content to end users. It leverages Rust's macro 
system to provide interfaces to GraphQL specifications in a way which feels natural and (hopefully) 
intuitive – do open an issue if you find that it isn't :)

Fair warning, however – Myoxine is very experimental, so any use is at your peril (the license means
that usage will always be at your peril, but at this stage it's especially new so as a result usage
is particularly likely to result in your peril).

> Myoxine is distributed subject to the Affero General Public License. Any use of this code
> may only be in compliance with the license.

We additionally ask that you use this software only for positive effect to improve people's lives.
We do not mandate this – it is merely a request. 

## Documentation

What use is software if you cannot use it? Documentation is included as doc-comments on all items
and integrated into the codebase. We also have separate documentation which we intend to host.
Because the library is not yet finished, we have not published the documentation.

## Browser support

Our intention is to support any browser correctly implementing the relevant web standards. Our
reference implementation is Mozilla Firefox. In the event that you run into browser compatibility 
issues, please raise an issue with the relevant browser vendor. 

We suggest that you encourage the people who use your software to use Firefox because (a) it is a 
better browser and (b) it offers better WebAssembly support with things like `wasmtime`.

## Chat

We intend to set up a Zulip server down the line. We might also ask the Yew folks if they might give
us a channel on their Discord server.
