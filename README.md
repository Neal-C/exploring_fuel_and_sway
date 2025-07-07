# Exploring Fuel and Sway

https://fuel.network/

https://docs.fuel.network/docs/intro/what-is-fuel/

https://github.com/FuelLabs/sway

tried version: fuelup 0.27.3 with forc 0.68.9

## Raison d'être

The FuelVM learns from the Ethereum ecosystem. It implements improvements suggested to the Ethereum VM (EVM) for many years that couldn’t be implemented due to the need to maintain backward compatibility, including parallel transaction execution and multiple native assets.

## Instructions

### With Docker


```bash
git clone https://github.com/Neal-C/exploring_fuel_and_sway.git
```

```bash
docker build -t neal-c-sway:latest .
```

```bash
docker run --name neal-c-sway neal-c-sway:latest
```

### With local installation

Requirements: Rust >= 1.88.0, fuelup = 0.27.3, forc = 0.68.9

Rust installation instructions: https://www.rust-lang.org/tools/install

fueulup installation instructions: https://docs.fuel.network/guides/installation/

forc installation instructions: should be installed with fuelup

```bash
git clone https://github.com/Neal-C/exploring_fuel_and_sway.git
```

```bash
cd exploring_fuel_and_sway
```

Build

```bash
forc build
```

Run the tests 

```bash
cargo test
```

### Deploy To Testnet

Now, you can deploy the contract to the latest testnet with the forc deploy command.

```bash
forc deploy --testnet
```

## Resources

- https://docs.fuel.network/docs/sway/introduction/getting_started/
- https://docs.fuel.network/guides/counter-dapp/