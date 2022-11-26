export enum ConnectionType {
    Mongo, Postgres, Redis, None
}

export class GenericConnection{
    name: string;
    db_type: ConnectionType;

    constructor(name: string, db_type: ConnectionType){
        this.name = name;
        this.db_type = db_type;
    }
}