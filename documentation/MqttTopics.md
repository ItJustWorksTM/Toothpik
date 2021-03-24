## Toothpik MQTT topic specification

Here you will find all mqtt topics used in the Toothpik system including the body format.

Requests, e.g. any publishes the store components subscribe to can have a JSON or CBOR body and the response will be the same format.
Unless specified otherwise, all other bodies are CBOR encoded fixed-length maps with string keys, i.e realtime updates like the dentist registry.  

### Reused structures

Appointment structure:
  - id: String (UUIDv4)
  - patient\_id: String (UUIDv4)
  - reason: String
  - mobile\_number: String
  - email: String
  - patient name: String
  - time: Integer: String (format "YYYY-MM-DD hh:mm")
  - dentist\_id: String (UUIDv4)

Clinic structure:
  - id: String (UUIDv4
  - name: String
  - dentists: Array of String (each a UUIDv4)
  - address: String
  - city: String
  - coordinate\_long: Float
  - coordinate\_lat: Float

User structure:
  - id: UUIDv4
  - name: String
  - username: String

Availability structure:
  - dentistid: UUIDv4
  - availability: Array of
    - date: String (format "YYYY-MM-DD")
    - time: Array of
      - time: String (format "hh:mm")
      - available: Bool

AvailabilityUpdate structure:
  - dentistid: UUIDv4
  - date: String (format "YYYY-MM-DD")
  - time: String (format "hh:mm")
  - available: Bool

Credentials structure:
  - user\_id: String (UUIDv4)
  - username: String
  - secret: String

### Patient client

**Publish**

- store/user/$anonid/my\_id <~> requests user id for supplied username
  * Preconditions
    - Client is connected with the client identifier $anonid
    - Client is connected with a username/secret combination
  * Body (Always)
    - username: string

- store/user/$uuid/self <~> requests or replaces the current data
  * Preconditions
    - Client is connected with a username/secret combination
    - Client's user account has identifier $uuid
  * Body (Request): Empty
  * Body (Replace): User structure

- store/user/public/$anonid/register <~> creates a new user on the network
  * Preconditions
    - Client is connected with the client identifier $anonid
    - Client is connected anonymously
  * Body (Always): User structure

- store/dentist/public/$uuid/registry <~> requests dentist registry
  * Preconditions
    - Either
      - Client is connected with a username/secret combination
      - Client's user account has identifier $uuid
    - Or
      - Client is connected anonymously with client identifier $uuid
  * Body (Always): Empty

- store/appointment/$uuid/book <~> book appointment with full info
  * Preconditions
    - Client is connected with a username/secret combination
    - Client's user account has identifier $uuid
  * Body (Always): Appointment structure

- store/appointment/$uuid/quick\_book <~> book appointment with minimal info
  * Preconditions
    - Client is connected with a username/secret combination
    - Client's user account has identifier $uuid
  * Body (Always)
    - userid: String (UUIDv4)
    - requestid: Integer
    - dentistid: String (UUIDv4)
    - issuance: Integer  
    - time: String (format "YYYY-MM-DD hh:mm")

- store/appointment/$anonid/public/availability
  * Preconditions
    - Client is connected anonymously
  * Body (Always)
    - dentistid: UUIDv4
    - start_date: String (format "YYYY-MM-DD")
    - end_date: String (format "YYYY-MM-DD")

- store/user/$uuid/validate
  * Preconditions
    - Client is connected anonymously
  * Body (Always)
    - username: String
    - verification\_code: Integer

**Subscribe**

- client/$anonid/reply/store/user/my\_id
- client/$uuid/reply/store/user/self
- client/$anonid/reply/store/user/register
- client/$uuid/reply/store/dentist/registry
- client/$uuid/reply/store/appointment/quick\_book
- client/$uuid/reply/store/appointment/book
- client/$anonid/reply/store/user/validate
- store/dentist/public/realtime/registry

### User store

**Publish**

- auth/reply/$auid/store/user/secret
  * Preconditions
    - Store is connected
  * Body (Success): Credentials structure
  * Body (Failure)
    - username: String
    - error: null

- auth/cache/add <~> add these credentials to the cache
  * Preconditions
    - Store is connected
  * Body (Always): Credentials structure

- auth/cache/del <~> remove this user from cache (may be triggered by a secret change)
  * Preconditions
    - Store is connected
  * Body (Always)
    - user\_id: String (UUIDv4)

- auth/cache/flush <~> flush all cache entries
  * Preconditions
    - Store is connected
  * Body (Always): Empty

- client/$anonid/reply/store/user/my\_id
  * Preconditions
    - Store is connected
  * Body (Always)
    - id: String (UUIDv4)

- client/$uuid/reply/store/user/self <~> provides data on self
  * Preconditions
    - Store is connected
  * Body (Success): user structure  
  * Body (Failure)
    - error: String (error message)

- client/$anonid/reply/store/user/validate
- client/$anonid/reply/store/user/register
  * Preconditions
    - Store is connected
  * Body (Success): user structure  
    - id: String (UUIDv4)
  * Body (Failure)
    - error: String (error message)

**Subscribe**

- store/user/$auid/secret
- store/user/$anonid/my\_id
- store/user/$uuid/self
- store/user/public/$anonid/register
- store/user/$uuid/validate

### Dentist store

**Publish**

- client/$uuid/reply/store/dentist/registry
  * Preconditions
    - Store is connected
  * Body (Always): Array of Clinic structures (all clinics)

- store/dentist/public/realtime/registry
  * Preconditions
    - Store is connected
  * Body (Always): Array of Clinic structures (all clinics)

**Subscribe**

- store/dentist/public/$uuid/registry

### Authenticator

**Publish**

- store/user/$anonid/secret
  * Preconditions
    - Authenticator is connected
  * Body (Always)
    - username: String

**Subscribe**

- auth/$anonid/reply/store/user/secret
- auth/cache/add
- auth/cache/del
- auth/cache/flush

### Appointment store

**Publish**
- store/appointment/public/realtime/availability
  * Preconditions
    - Store is connected
  * Body (Always):
    - AvailabilityUpdate structure

- client/$anonid/reply/store/appointment/availability/$dentistid
  * Preconditions
    - Store is connected
  * Body (Success): Availability structure
  * Body (Failure)
    - error: String (error message)

- client/$uuid/reply/store/appointment
  * Preconditions
    - Store is connected
  * Body (Always): Array of Appointments structures (appointments of the requester)

- client/$uuid/reply/store/appointment/book
- client/$uuid/reply/store/appointment/quick\_book
  * Preconditions:
    - Store is connected
  * Body (Always)
    - userid: String (UUIDv4)
    - requestid: Integer
    - time: String ("none" on failed booking)

- store/dentist/public/$uuid/registry
  * Preconditions:
    - Store is connected
  * Body (Always): Empty

**Subscribe**
- store/appointment/public/realtime/availability
- store/appointment/$uuid
- store/appointment/$uuid/book/cuuid
- store/appointment/$uuid/quick\_book
- client/$uuid/reply/store/dentist/registry
