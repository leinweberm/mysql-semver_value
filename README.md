# mysql-semver-value
- Convert semantic version value into comparison and order compatible string.
- Supporting values from '0' to '4294967295.4294967295.4294967295.4294967295'

## fn semver_value
```shell
semver_value(version: str, segments: int)
```
**params:**
- version: string, not null. Each segment can represent up to uint32 value
- segments: number of segments in version string. Support values between 1 and 4
- *working prototype in development stage
 
** returns:**
- VARBINARY(28) representing string of 39 length

## register plugin
1. build plugin from source code:
```shell
cargo build --release
```

2. (Optional) run mysql database in docker container
```shell
docker run -d --name mysql-semver -e MYSQL_ROOT_PASSWORD=rootpassword -e MYSQL_DATABASE=testdb -v /$PATH_TO_PLUGIN:/usr/lib64/mysql/plugin -p 3306:3306 mysql
```
volume location may be different base on operating system, you can verify it running this query:
```sql
SHOW VARIABLES LIKE 'plugin_dir';
```

3. connect to mysql database
```shell
docker exec -it mysql-semver mysql -u root -p
```

4. register UDF
```sql
CREATE FUNCTION semver_value RETURNS STRING SONAME 'libmysql_semver.so';
```

5. use function in queries
```sql
SELECT CAST(semver_value('1.0.259', 3) AS CHAR(39));
```
