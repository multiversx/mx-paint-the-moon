# Paint the Moon
This is a blockchain game made in Rust on MultiversX. The project contains 2 smart contracts, frontend and several microservices.

### The idea
Inspired by `r/place`, we created a game in which one player can paint one pixel on the moon at a specific time. The resource used to paint a point into a specific color can be harvested and claimed once a time delay has passed if the user registered for harvesting.

### Technical details

## The map
The map is a sphere with generated fixed points. Using perspective projection, we can represent 3D spaces into 2D coordinates. These 2D coordinates are stored into the `paint-the-moon` smart contract, along with the state (color) of each point at a given time.

## The frontend
The frontend receives the 2D coordinates and translates them into 3D coordinates and renders the map. All the sc requests' results are received from the microservice.

## Microservice
The microservice queries the smart contract once in a while, caches the information and prepares the information format for the frontend.