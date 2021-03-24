# Toothpik Dentist Store
A component that manages and fetches the dentist registry.

#### Building
Be sure to install [Rust](https://www.rust-lang.org/tools/install) first.
- ```git clone <repo url>```
- ```cd appointment-store```
- ```git submodule update --init --recursive```
- ```cargo build```

#### Running
- Start a MQTT broker e.g. `mosquitto -v`
- OR you can also set ```BROKER_URL``` to connect to a non localhost broker.
- ```cargo run```

#### Developing note
We try to share code where ever possible using our [rust store utils](https://git.chalmers.se/courses/dit355/2020/group-4/rust-store-utils) repo.  
As this is a git submodule do remember to regularly update it with ```git submodule update --recursive```.
