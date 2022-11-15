
interface DatabaseOptions{
    username: string;
    password: string | null;
    host: string;
    port: string;
    database?: string;
}


class MongoOptions implements DatabaseOptions{
    username: string;
    password: string | null;
    host: string;
    port: string;

    constructor(username: string, password: string | null, host: string, port: string) {
        this.username = username;
        this.password = password;
        this.host = host;
        this.port = port;
    }
}

class PostgresOptions implements DatabaseOptions {
    username: string;
    password: string | null;
    host: string;
    port: string;
    database: string;

    constructor(username: string, password: string | null, host: string, port: string, database: string) {
        this.username = username;
        this.password = password;
        this.host = host;
        this.port = port;
        this.database = database;
    }
}

class RedisOptions implements DatabaseOptions{
    username: string;
    password: string | null;
    host: string;
    port: string;

    constructor(username: string, password: string | null, host: string, port: string) {
        this.username = username;
        this.password = password;
        this.host = host;
        this.port = port;
    }
}




export {
    MongoOptions, PostgresOptions, RedisOptions
};
export type { DatabaseOptions };
