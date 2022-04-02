CREATE TABLE IF NOT EXISTS `tbl_user` (
  `uuid` bigint(20) NOT NULL AUTO_INCREMENT,
  `userID` varchar(100) DEFAULT NULL,
  `userPW` varchar(50) DEFAULT NULL,
  `nickname` varchar(100) DEFAULT NULL,
  `exceptArena` int(11) DEFAULT NULL,
  `regLastLoginDate` datetime DEFAULT CURRENT_TIMESTAMP,
  `regDate` datetime DEFAULT CURRENT_TIMESTAMP,
  `walletAddress` varchar(100) DEFAULT NULL,
  `verifyEmailHash` varchar(100) DEFAULT NULL,
  `verifyEmail` BOOL DEFAULT,
  `txHash ` varchar(100) DEFAULT NULL,
  PRIMARY KEY (`uuid`),
  KEY `userID` (`userID`),
  KEY `userPW` (`userPW`),
  KEY `nickname` (`nickname`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb3;

