# Toothpik - a look on the safety

### Abstract

One of the main drivers while developing _Toothpik_ was system security.  
While we do not expect potential customers to deploy _Toothpik_ in production with the built-in safety mechanisms, we still wanted to offer decent protection against low-effort attacks.  
Due to shortcomings regarding our development schedule and the unexpected obscurity on the documentation of some of the libraries we use, we were not able to seal the system completely. This document will explain what security features there are, what gaping holes in our safety there are, as well as potential solutions to fix these major issues, that would make the system safe to be deployed in production.


### Features

We consider that anyone connected to the system is a user of the system, or a system component itself (or someone trying to spoof one of those two).  
From that we derive two categories of security features: the ones that are meant to prevent attackers to spoof their identity, and the ones that are meant to restrict the information accessible from within the system (we will refer to the former case as "external" and the former as "internal" security measures").

#### Internal safety

For internal safety, we use specific patterns on the MQTT topics used. This allows us to perform client/server-like messaging, and keep unauthorized users from accessing restricted information.

We have the following layers:
- Anonymous: the users which have not logged in; they should only have access to public information.
- User: logged in users. They should have access to all restricted information sent to them.
- System: system components. Should have access to all traffic they are supposed to handle.

To allow client components to make requests and server components to reply to them, the following topic patterns rules are used:
- The topic must start with the type of destination: "client/" for a client, "store/" for stores, and "auth/" for the authenticator.
- The topic must then be followed by the name of the destination
  * for stores: "user", "dentist", or "appointment"
  * for logged-in users: their UUID as stored by the user store
  * otherwise: the MQTT connection id
- The remaining part of the topic depends on the message type:
  *  request: the origin uuid, then the target endpoint (preceded by "public" if this is a public endpoint).
  *  response: "reply/", followed by the request topic stripped of its origin uuid.

These patterns were designed to make it easy to write patterns for accepting and rejecting subscriptions and publications on unauthorized topics.  
Note: there might be inconsistencies with the topics listed in the documentation. With the exception of realtime events, they should be considered bugs.

Those patterns are hardcoded into the authenticator as rules, and you should reimplement these rules if you wish to use another broker for deployment.  
In our implementation, the following is true:
- Requests never target system clients
- An anonymous client may not make requests to private endpoints.
- A client may not make requests to private endpoints as someone else
- A client may not publish data to other clients.

As far as we have been able to tell, these rules properly ensure the safety they have been designed for (with a minor exception on public routes which will be detailed later in this document).

Alternatives to enforcing these rules with an authenticator include:
- Use a purpose-built client/server protocol (not chosen due to the system requirement to use MQTT)
- Encrypt all traffic with asymmetric encryption (not chosen due to complexity, performance slowdown, harder diagnosability, and quantum-crypto attacks)


#### External safety

Our external safety relies solely on controlling MQTT connection requests.

The authenticator is identified by its MQTT connection id (they are unique as guaranteed by the broker). 
Stores are restricted by name and IP. The name must start by "store-", and the IP must be `127.0.0.1`. If you wish to deploy a store over the network, we recommend using SSH port-forwarding.  
Normal users do provide credentials. Their real credentials are checked by username against the ones stored on the user store, and only if valid the connection request is accepted.  
Anonymous users are identified as such because they do not provide credentials on their MQTT CONNECT. They are always authorized but flagged as anonymous for the internal safety mechanism.  
Note: to somewhat protect users against phishing, we force them to use two factor authentication, in the form of a time-based one-time token (the most common implementation being Google Authenticator).  

The most accessible alternative to this would probably be using OAuth2 on MQTT 5.x such that the "Bearer" field is an MQTT user property, and delegating OAuth2 authorization to a thirdparty service.

We also try to protect the system against volumetric spam (DoS attacks mostly).  
We do so by equipping the stores with a rate limiter which drops all extra packets, as well as a couple restrictions on registrations:
- a captcha, using the HCaptcha service
- email verification, via an SMTP+STARTTLS connection to an outside service.
These restrictions are both compile-time optionally enabled features.

### Issues

The most glaring issue in the system is the inability to safely connect stores over the network without using external software.  
What was originally intended was to use TLS client authentication for stores, where the broker would check that the certificates sent by the stores are signed with the one the broker possesses.  
We believed this would have been a fitting and secure solution, however we were not able to implement it since the MQTT library used for our stores lacks documentation for TLS parameters, and our attempts at making it work proved unfruitful.

The other big issue is that we store plaintext passwords. The correct solution would be to store salted hashes with an up-to-date crypto hashing algorithm (such as SHA3).

One medium issue is regarding spam: someone could easily register an account and make an automated system which books all the available slots. One remedy would be to use a captcha on the booking page. We did not implement it due to immutable requirements from the product-owner concerning the booking request format.

Finally, the minor issue mentionned before: if an anonymous user's MQTT connection id is the same as a logged-in user's system id, they will see the replies to each other's requests to public endpoints. We do not judge that to be a security issue, but we would understand if you did. This could be solved simply by making connection ids and system ids a different format.

### Conclusion

With some more effort Toothpik could be made production-safe, at which point it would be wise to ask a thirdparty to perform a full security audit.
As-is we believe it stands safe for deploying in a sandbox.
