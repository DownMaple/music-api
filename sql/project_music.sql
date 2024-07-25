/*
 Navicat Premium Data Transfer

 Source Server         : 本机
 Source Server Type    : MySQL
 Source Server Version : 80012 (8.0.12)
 Source Host           : localhost:3306
 Source Schema         : project_music

 Target Server Type    : MySQL
 Target Server Version : 80012 (8.0.12)
 File Encoding         : 65001

 Date: 25/07/2024 17:18:12
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for music
-- ----------------------------
DROP TABLE IF EXISTS `music`;
CREATE TABLE `music`  (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '歌曲id',
  `img` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '歌曲封面',
  `title` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '歌曲名称',
  `author_id` int(11) NULL DEFAULT NULL COMMENT '歌曲作者 id',
  `author_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '歌曲作者',
  `album_id` int(11) NULL DEFAULT NULL COMMENT '专辑 id',
  `album_title` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '专辑',
  `issued_time` datetime NOT NULL COMMENT '发行日期',
  `duration` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '歌曲时长',
  `link` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '歌曲路径',
  `classify_id` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '歌曲分类，id，多个',
  `classify_title` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '歌曲分类，分类名称， 多个',
  `sort` int(11) NOT NULL DEFAULT 1 COMMENT '歌曲排序',
  `views` int(11) NOT NULL DEFAULT 0 COMMENT '播放次数',
  `status` int(11) NOT NULL DEFAULT 1 COMMENT '歌曲状态， 1 正常， 0 下架 ， 2待审核',
  `create_time` datetime NULL DEFAULT NULL COMMENT '创建时间',
  `create_id` int(11) NULL DEFAULT NULL COMMENT '创建人',
  `update_time` datetime NULL DEFAULT NULL COMMENT '修改时间',
  `update_id` int(11) NULL DEFAULT NULL COMMENT '修改人',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = MyISAM AUTO_INCREMENT = 4 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of music
-- ----------------------------
INSERT INTO `music` VALUES (1, 'static/img/%E4%BD%A0%E5%B0%B1%E6%98%AF%E8%BF%9C%E6%96%B9.jpeg', '你就是远方', 243, '就是南方凯', 605, '你就是远方', '2002-10-06 03:03:30', '03:36', '/static/music/%E5%B0%B1%E6%98%AF%E5%8D%97%E6%96%B9%E5%87%AF-%E4%BD%A0%E5%B0%B1%E6%98%AF%E8%BF%9C%E6%96%B9.flac', 'fVU1Rh5NXV', '说唱', 10, 567, 1, '2022-09-02 13:15:39', 123, '2022-07-08 11:31:47', 41);
INSERT INTO `music` VALUES (2, 'static/img/这城市风总是很大.jpeg', '这城市风总是很大', 2, '枯木逢春', 2, '这城市风总是很大', '2024-01-06 03:03:30', '04:07', 'static/music/枯木逢春%20-%20这城市风总是很大.mp3', '2', '流行歌曲', 1, 1, 1, '2022-09-02 13:15:39', NULL, '2024-07-15 17:31:45', NULL);
INSERT INTO `music` VALUES (3, '测试img', '测试新增音乐', NULL, NULL, NULL, NULL, '2024-01-06 03:03:30', NULL, 'static/music/就是南方凯-你就是远方.flac/static/music/%E5%B0%B1%E6%98%AF%E5%8D%97%E6%96%B9%E5%87%AF-%E4%BD%A0%E5%B0%B1%E6%98%AF%E8%BF%9C%E6%96%B9.flac', NULL, NULL, 1, 0, 1, '2024-07-15 17:29:41', NULL, '2024-07-15 17:29:41', NULL);

-- ----------------------------
-- Table structure for music_song_list
-- ----------------------------
DROP TABLE IF EXISTS `music_song_list`;
CREATE TABLE `music_song_list`  (
  `id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '关联表id',
  `song_list_id` int(11) NOT NULL COMMENT '歌单表id',
  `music_id` int(11) NOT NULL COMMENT '音乐id',
  `create_time` datetime NULL DEFAULT NULL,
  `create_id` int(11) NULL DEFAULT NULL,
  `update_time` datetime NULL DEFAULT NULL,
  `update_id` int(11) NULL DEFAULT NULL,
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = MyISAM AUTO_INCREMENT = 3 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Fixed;

-- ----------------------------
-- Records of music_song_list
-- ----------------------------
INSERT INTO `music_song_list` VALUES (1, 1, 1, '2024-07-25 10:05:42', 1, '2024-07-25 10:05:45', 1);
INSERT INTO `music_song_list` VALUES (2, 1, 2, '2024-07-25 10:05:55', 1, '2024-07-25 10:05:58', 1);

-- ----------------------------
-- Table structure for song_list
-- ----------------------------
DROP TABLE IF EXISTS `song_list`;
CREATE TABLE `song_list`  (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '歌单id',
  `img` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '歌单封面',
  `author` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '歌单创作者',
  `title` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '歌单名称',
  `description` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '歌单描述',
  `classify_title` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '歌单分类',
  `classify_id` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '歌单分类id',
  `status` int(11) NULL DEFAULT 1 COMMENT '歌单状态',
  `create_time` datetime NULL DEFAULT NULL,
  `create_id` int(11) NULL DEFAULT NULL,
  `update_time` datetime NULL DEFAULT NULL,
  `update_id` int(11) NULL DEFAULT NULL,
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = MyISAM AUTO_INCREMENT = 2 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of song_list
-- ----------------------------
INSERT INTO `song_list` VALUES (1, 'static/img/%E4%BD%A0%E5%B0%B1%E6%98%AF%E8%BF%9C%E6%96%B9.jpeg', '官方歌单', '每日30首', '官方推荐，最为致命', NULL, NULL, 1, '2024-07-24 16:25:07', 1, '2024-07-24 16:25:10', 1);

SET FOREIGN_KEY_CHECKS = 1;
