DROP DATABASE IF EXISTS restaurant;
CREATE DATABASE restaurant;
USE restaurant;

CREATE TABLE `tables` (
	`id` VARCHAR(64) PRIMARY KEY,
	`number` INT UNSIGNED NOT NULL UNIQUE,
	`created_at` DATETIME NOT NULL,
	`updated_at` DATETIME NOT NULL
);

CREATE TABLE `items` (
	`id` VARCHAR(64) PRIMARY KEY,
	`name` VARCHAR(255) NOT NULL,
	`preparation_time` DATETIME NOT NULL,
	`created_at` DATETIME NOT NULL,
	`updated_at` DATETIME NOT NULL
);