# Welcome to the Trust Machines Interview Project!

This is not the original README file, the original now lives [here](docs/README.md).

## Build

Check out this code, cd to the root level of it and in your terminal type:
```
rustup update && cargo update
cargo build
```

### Setup Database

My code uses an aws dynamodb table I set up, so **you don't have to worry about setting up anything**. TaDa!  
If you want to access it because you are curious, you can do so with this [link](https://us-west-2.signin.aws.amazon.com/oauth?client_id=arn%3Aaws%3Asignin%3A%3A%3Aconsole%2Fdynamodb&code_challenge=sCojp5d-Qs8dhVzBPR--sPUAczE_7F2qP9qEf7tykiI&code_challenge_method=SHA-256&response_type=code&redirect_uri=https%3A%2F%2Fus-west-2.console.aws.amazon.com%2Fdynamodbv2%2Fhome%3FhashArgs%3D%2523edit-item%253FitemMode%253D2%2526pk%253Dbf28bdee-5aa7-4f08-8a92-4379ccf3adb1%2526route%253DROUTE_ITEM_EXPLORER%2526sk%253D%2526table%253Dcampaigns%26isauthcode%3Dtrue%26oauthStart%3D1724294084061%26region%3Dus-west-2%26state%3DhashArgsFromTB_us-west-2_78310a3dee88a72e).

Here are the credentials you'll need - don't tell anybody (:

Account ID  
IAM user name  
Password

772399174060  
trust-machines-dynamodb  
trust-machines-dynamodb-UC@@

## Run

```
cargo run
```

## Test - Integration and E2E

In one tab of your terminal start the server (type `cargo run`).
In a different tab type:
```
cargo test
```

## Test Manually in Postman

Postman doesn't allow sharing with free accounts like mine, so you'll have to grab the `/docs/trust-machines-*.json` files and import them into your Postman workspace first.

## Design Document

I choose to re-write my [original design document](https://docs.google.com/document/d/1vbQbLUEhgL9xoYl_26UvL48peRjXsbvgWJQ68VMlZSY/edit#heading=h.sd4bve46zbmv) to better reflect what I ended up building.  

An improved and updated Design Document is here: https://docs.google.com/document/d/1n9Xo5PnE2DdGQWe7MFj6ibI2p5fLfqPCR1aTw4bDc5M/edit


## A Moment Of Gratitude

Thank you for reading this and for answering my questions with the project. If you have any issues - feel free to ping me on Discord or email me at esager@gmail.com

Ellie Sager
