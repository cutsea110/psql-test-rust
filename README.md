# postgresql sample written in rust

ref.) https://diesel.rs/guides/getting-started


## run postgres server

```
$ docker-compose up -d
```

## commands

### check to connect sampledb

```
$ psql -h localhost -p 15432 -U admin sampledb
```

### write post as draft

```
$ cargo run --bin write_post
```

### publish post

```
$ cargo run --bin publish_post [post id]
```

### show posts

```
$ cargo run --bin show_posts
```

### delete post

```
$ cargo run --bin delete_post [post title]
```
