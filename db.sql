USE `tutorial`;

/*Table structure for table `course` */

DROP TABLE IF EXISTS `course`;

CREATE TABLE `course` (
  `id` int NOT NULL AUTO_INCREMENT,
  `teacher_id` int NOT NULL,
  `name` varchar(140) COLLATE utf8_bin DEFAULT NULL,
  `time` datetime DEFAULT CURRENT_TIMESTAMP,
  `description` varchar(2000) COLLATE utf8_bin DEFAULT NULL,
  `format` varchar(30) COLLATE utf8_bin DEFAULT NULL,
  `structure` varchar(200) COLLATE utf8_bin DEFAULT NULL,
  `duration` varchar(30) COLLATE utf8_bin DEFAULT NULL,
  `price` int DEFAULT NULL,
  `language` varchar(30) COLLATE utf8_bin DEFAULT NULL,
  `level` varchar(30) COLLATE utf8_bin DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=25 DEFAULT CHARSET=utf8mb3 COLLATE=utf8_bin;

/*Data for the table `course` */

insert  into `course`(`id`,`teacher_id`,`name`,`time`,`description`,`format`,`structure`,`duration`,`price`,`language`,`level`) values (1,1,'course name changed','2022-05-10 15:05:33','updated course','','','',0,'中文','medium'),(2,2,'math','2022-05-10 15:05:51',NULL,NULL,NULL,NULL,NULL,NULL,NULL),(16,2,'Second course','2022-05-11 02:32:17',NULL,NULL,NULL,NULL,NULL,NULL,NULL),(17,1,'Test course','2022-05-11 13:31:45',NULL,NULL,NULL,NULL,NULL,NULL,NULL),(18,1,'Test course','2022-05-11 13:44:14',NULL,NULL,NULL,NULL,NULL,NULL,NULL),(23,2,'Second course','2022-05-14 03:39:41',NULL,NULL,NULL,NULL,NULL,NULL,NULL);

/*Table structure for table `teacher` */

DROP TABLE IF EXISTS `teacher`;

CREATE TABLE `teacher` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(100) COLLATE utf8_bin NOT NULL,
  `picture_url` varchar(200) COLLATE utf8_bin DEFAULT NULL,
  `profile` varchar(2000) COLLATE utf8_bin DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=10 DEFAULT CHARSET=utf8mb3 COLLATE=utf8_bin;

/*Data for the table `teacher` */

insert  into `teacher`(`id`,`name`,`picture_url`,`profile`) values (2,'Test Teacher','www.test_url.com','test profile'),(4,'test teacher','www.test.com',NULL),(5,'test teacher','www.test.com',NULL),(6,'test teacher','www.test.com',NULL),(7,'test teacher','www.test.com',NULL);

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
