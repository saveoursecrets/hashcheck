# HIBP

List of hashes of known breached password downloaded from the [haveibeenpwned database](https://haveibeenpwned.com/) using the [easypwned](https://github.com/easybill/easypwned) downloader.

Run the `download.sh` script to install the downloader and download the database and create the bloom filter.

Run the `upload.sh` script to copy the downloaded database and bloom filter to S3.

## Docker

### Build

```
docker build . --tag hibp
```

### Run

```
docker run -it --rm --name hibp-service -p 3342:3342 hibp
```

© Copyright Save Our Secrets Pte Ltd 2022; all rights reserved.
