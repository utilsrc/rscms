// 初始化数据库和用户
db = db.getSiblingDB('admin');
db.auth('root', 'example');

// 创建应用数据库
db = db.getSiblingDB('myapp');

// 创建应用专用用户
db.createUser({
    user: 'appuser',
    pwd: 'apppassword',
    roles: [
        { role: 'readWrite', db: 'myapp' },
        { role: 'dbAdmin', db: 'myapp' }
    ]
});

// 初始化集合和数据
db.createCollection('users');
db.users.insertMany([
    { name: 'Alice', email: 'alice@example.com', createdAt: new Date() },
    { name: 'Bob', email: 'bob@example.com', createdAt: new Date() }
]);

// 创建索引
db.users.createIndex({ email: 1 }, { unique: true });
