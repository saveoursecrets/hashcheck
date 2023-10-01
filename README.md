# Hash Check

Check a password hash (SHA-1) against a list of hashes of known breached password downloaded from the [haveibeenpwned database](https://haveibeenpwned.com/) using the [easypwned](https://github.com/easybill/easypwned) downloader.

## API

Service meta data is available at `GET /`.

### Check

The service provides a single GET endpoint:

```
/:hash
```

Where `:hash` is a SHA-1 hash of the password to check; the reply is a JSON boolean `true` if the password hash was found in the database otherwise `false`.

## Scripts

### Download

Run the `scripts/download.sh` script to install the downloader, download the database and create the bloom filter.

### Upload

Run the `scripts/upload.sh` script to copy the bloom filter and meta data files to S3.

## Docker

### Build

If you have already downloaded the database then using `-f Dockerfile-local` will be faster as it won't download the bloom filter data from S3.

```
docker build . --tag hashcheck
```

### Run

```
docker run -it --rm --name hashcheck -p 3342:3342 hashcheck
```

© Copyright Save Our Secrets Pte Ltd 2022; all rights reserved.
