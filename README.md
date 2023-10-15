# Poker Engine

A quick Texas Hold'em poker engine to test out calling python from Rust.

The idea is that the core game engine will handle game state and calculating results of player actions while deferring to python scripts which are responsible for determining which actions to take.
Most AI/ML work is done in python so this is meant to mimic a tech stack where the core engine is written in more performant language, exposing hooks for scientists to work with and iterate on models running in Python. 
