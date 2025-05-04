// 切换到 admin 数据库并认证
db = db.getSiblingDB('admin');
db.auth('root', 'example');

// 认证后创建应用专用数据库
db = db.getSiblingDB('rscms');
// 创建应用专用用户
db.createUser({
    user: 'rscms',
    pwd: 'rscms',
    roles: [
        { role: 'readWrite', db: 'rscms' },
        { role: 'dbAdmin', db: 'rscms' }
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
