-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `tbl_auth` (
  `seq` bigint(20) NOT NULL AUTO_INCREMENT,
  `email` varchar(100) DEFAULT NULL,
  `password` varchar(200) DEFAULT NULL,
  `walletAddress` varchar(100) DEFAULT NULL,
  `verifyEmailHash` varchar(200) DEFAULT NULL,
  `verifyEmail` tinyint(1) DEFAULT NULL,
  `txHash` varchar(100) DEFAULT NULL,
  `regDate` datetime DEFAULT NULL,
  PRIMARY KEY (`seq`),
  KEY `email` (`email`),
  KEY `password` (`password`)
) ENGINE=InnoDB AUTO_INCREMENT=1000 DEFAULT CHARSET=utf8mb3;

CREATE TABLE IF NOT EXISTS `tbl_user` (
  `uuid` bigint(20) NOT NULL,
  `nickname` varchar(100) DEFAULT NULL,
  `exceptArena` int(11) DEFAULT NULL,
  `profileImage` varchar(100) DEFAULT NULL,
  `regLastLoginDate` datetime DEFAULT NULL,
  `regDate` datetime DEFAULT NULL,
  PRIMARY KEY (`uuid`),
  KEY `nickname` (`nickname`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3;

