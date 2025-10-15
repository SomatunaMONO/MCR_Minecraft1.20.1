val n = <item:minecraft:netherite_ingot>;
val b = <item:sophisticatedbackpacks:diamond_backpack>;
val d = <item:minecraft:diamond>;

val craftingResult = <item:sophisticatedbackpacks:netherite_backpack>;

val recipename = "easly_upgradebuckpack";

craftingTable.addShapeless(recipename, craftingResult,
    [ n, b, d ]
);

craftingTable.remove(<item:sophisticatedbackpacks:stack_upgrade_starter_tier>);

craftingTable.addShaped("easy_stack_upgrade_starter_tierb", <item:sophisticatedbackpacks:stack_upgrade_starter_tier>, [
    [<item:minecraft:copper_ingot>, <item:minecraft:copper_ingot>, <item:minecraft:copper_ingot>],
    [<item:minecraft:copper_ingot>, <item:sophisticatedbackpacks:upgrade_base>, <item:minecraft:copper_ingot>],
    [<item:minecraft:copper_ingot>, <item:minecraft:copper_ingot>, <item:minecraft:copper_ingot>]
]);

craftingTable.remove(<item:sophisticatedbackpacks:stack_upgrade_tier_1>);

craftingTable.addShaped("easy_stack_upgrade_tier_1b", <item:sophisticatedbackpacks:stack_upgrade_tier_1>, [
    [<item:minecraft:iron_ingot>, <item:minecraft:iron_ingot>, <item:minecraft:iron_ingot>],
    [<item:minecraft:iron_ingot>, <item:sophisticatedbackpacks:upgrade_base>, <item:minecraft:iron_ingot>],
    [<item:minecraft:iron_ingot>, <item:minecraft:iron_ingot>, <item:minecraft:iron_ingot>]
]);

craftingTable.remove(<item:sophisticatedbackpacks:stack_upgrade_tier_2>);

craftingTable.addShaped("easy_stack_upgrade_tier_2b", <item:sophisticatedbackpacks:stack_upgrade_tier_2>, [
    [<item:minecraft:gold_ingot>, <item:minecraft:gold_ingot>, <item:minecraft:gold_ingot>],
    [<item:minecraft:gold_ingot>, <item:sophisticatedbackpacks:stack_upgrade_tier_1>, <item:minecraft:gold_ingot>],
    [<item:minecraft:gold_ingot>, <item:minecraft:gold_ingot>, <item:minecraft:gold_ingot>]
]);

craftingTable.remove(<item:sophisticatedbackpacks:stack_upgrade_tier_3>);

craftingTable.addShaped("easy_stack_upgrade_tier_3b", <item:sophisticatedbackpacks:stack_upgrade_tier_3>, [
    [<item:minecraft:diamond>, <item:minecraft:diamond>, <item:minecraft:diamond>],
    [<item:minecraft:diamond>, <item:sophisticatedbackpacks:stack_upgrade_tier_2>, <item:minecraft:diamond>],
    [<item:minecraft:diamond>, <item:minecraft:diamond>, <item:minecraft:diamond>]
]);

craftingTable.remove(<item:sophisticatedbackpacks:stack_upgrade_tier_4>);

craftingTable.addShaped("easy_stack_upgrade_tier_4b", <item:sophisticatedbackpacks:stack_upgrade_tier_4>, [
    [<item:minecraft:netherite_ingot>, <item:minecraft:netherite_ingot>, <item:minecraft:netherite_ingot>],
    [<item:minecraft:netherite_ingot>, <item:sophisticatedbackpacks:stack_upgrade_tier_3>, <item:minecraft:netherite_ingot>],
    [<item:minecraft:netherite_ingot>, <item:minecraft:netherite_ingot>, <item:minecraft:netherite_ingot>]
]);