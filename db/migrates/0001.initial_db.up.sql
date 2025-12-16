CREATE TABLE IF NOT EXISTS `users` (
    `id` bigint NOT NULL AUTO_INCREMENT COMMENT '主键',
    `name` varchar(32) DEFAULT '' COMMENT '姓名',
    `username` varchar(255) NOT NULL COMMENT '账号',
    `password` varchar(64) NOT NULL COMMENT '密码',
    `role_id` bigint NOT NULL COMMENT '角色ID',
    `identity_code` varchar(64) DEFAULT NULL COMMENT '身份证号',
    `phone` varchar(16) DEFAULT '' COMMENT '手机号',
    `email` varchar(32) DEFAULT '' COMMENT '邮箱',
    `sex` varchar(8) DEFAULT '' COMMENT '性别',
    `avatar` varchar(255) DEFAULT '' COMMENT '头像',
    `status` varchar(8) DEFAULT '' COMMENT '状态',
    `remark` varchar(255) DEFAULT '' COMMENT '备注',
    `created_at` datetime DEFAULT NULL,
    `create_by` bigint DEFAULT 0 COMMENT '创建人',
    `updated_at` datetime DEFAULT NULL,
    `update_by` bigint DEFAULT 0 COMMENT '更新人',
    `deleted_at` datetime DEFAULT NULL,
    PRIMARY KEY (`id`) USING BTREE,
    UNIQUE KEY `user_username` (`username`) USING BTREE
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = DYNAMIC COMMENT = '账号管理';


CREATE TABLE IF NOT EXISTS `sys_oper_log` (
    `oper_id` bigint(20) NOT NULL COMMENT '日志主键',
    `api_name` varchar(50) DEFAULT '',
    `method` varchar(100) DEFAULT '' COMMENT '方法名称',
    `request_method` varchar(10) DEFAULT '' COMMENT '请求方式',
    `oper_name` varchar(50) DEFAULT '' COMMENT '操作人员',
    `oper_url` varchar(255) DEFAULT '' COMMENT '请求URL',
    `oper_ip` varchar(128) DEFAULT '' COMMENT '主机地址',
    `oper_location` varchar(255) DEFAULT '' COMMENT '操作地点',
    `oper_param` varchar(2048) DEFAULT '' COMMENT '请求参数',
    `json_result` varchar(2048) DEFAULT '' COMMENT '返回参数',
    `status` char(1) DEFAULT '0' COMMENT '操作状态（0正常 1异常）',
    `error_msg` varchar(2000) DEFAULT '' COMMENT '错误消息',
    `oper_time` datetime DEFAULT NULL COMMENT '操作时间',
    `cost_time` bigint(20) DEFAULT '0' COMMENT '消耗时间'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='操作日志记录';