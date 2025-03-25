db = db.getSiblingDB("rscms");

db.createCollection("admins");
db.admins.insertMany([
    {
        username: "rscms-admin",
        // password: "rscms-admin"
        password: "$2b$12$rJMgvLmL60eVWEK8fhzJt.VwZPzFgGl0jfzfiSq1Ct0bsRDXA427a",
        last_login_ip: "",
        last_login_time: 0,
        is_locked: false,
        create_time: Math.floor(db.serverStatus().localTime.getTime() / 1000),
        update_time: Math.floor(db.serverStatus().localTime.getTime() / 1000),
        delete_time: 0,
        is_deleted: false,
    }
]);

print("Admins inilize success！");