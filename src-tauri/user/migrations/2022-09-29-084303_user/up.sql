-- Your SQL goes here
CREATE TABLE `user` (
        `id` char(36) NOT NULL COMMENT 'id',
        `name` varchar(64) NOT NULL COMMENT '姓名',
        `age` int(11) DEFAULT NULL COMMENT '年龄',
        `password` char(32) NOT NULL COMMENT '密码',
        PRIMARY KEY (`id`,`name`) /*T![clustered_index] NONCLUSTERED */
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;