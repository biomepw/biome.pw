CREATE TABLE `applications` (
    `row_id` INT NOT NULL AUTO_INCREMENT,
    PRIMARY KEY (`row_id`),
    `minecraft_username` text NOT NULL,
    `age` int(11) NOT NULL,
    `linking_id` bigint(20) NOT NULL,
    `add_one_thing` text NOT NULL,
    `projects_on_biome` text NOT NULL,
    `biggest_project` text NOT NULL,
    `showcase` text NOT NULL,
    `status` int(11) NOT NULL
) ENGINE = InnoDB DEFAULT CHARSET = latin1;