mediadex
========

index your media 

get started
-----------

clone and install::

    git clone git@github.com:haggisunk/mediadex.git && cd mediadex
    cargo run

begin::

    mediadex slurp

see::

    mediadex dump

design
------

mediadex is meant to process media content available at any uri and index it under various dimensions, in fact any you can devise.  a mediadex server maintains the index in a database.  query the server for a passthrough query to the backend database.  create templates for what media attributes you want catalogued and indexed.

server responds to api calls from clients by performing IO with a database.  choosing a relational database design on a flip of a coin as both types could be useful here.

get media file and compute identity (hash) (!! make sure to strip any metadata from the file before computing hash !!), referencing against index database for existing identity.  parse media metadata per template from sources:  cddb, file path, adjacent data files, id3 tags, filesystem tags?  store media source path for later culling, transfer, tagging, etc other processing.

management options exist to transfer & dedupe content into canonical media directory format with standardized media metadata, info, tagging, auxiliary resources.  and to transform among media dir format.
