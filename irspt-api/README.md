# irspt-api

This crate is for the API client that interfaces with "Portal das Finanças" tax authority website.\
It accomplished that by crawling the website using `WebDriver`.

Networking and infrastructure dependent code like this one should go into the `irspt-infra` crate,\
however, in this case, it was created reusable crate.
