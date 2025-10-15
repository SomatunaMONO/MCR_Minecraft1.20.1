const disableItemids = [
  "projecte:dm_pick",
  "projecte:dm_shovel",
  "projecte:dm_axe",
  "projecte:dm_hoe",
  "projecte:dm_sword",
  "projecte:watch_of_flowing_time",
  "projecte:dm_boots",
  "projecte:dm_leggings",
  "projecte:dm_chestplate",
  "projecte:dm_helmet",

  "projecte:dark_matter",
  "projecte:red_matter",
  "projecte:iron_band",
  "projecte:nova_cataclysm",
  "projecte:nova_catalyst",

  "sophisticatedbackpacks:everlasting_upgrade",

  "enderio:empty_soul_vial",
];

ServerEvents.recipes((event) => {
  // アイテム名で削除
  disableItemids.forEach((id) => {
    event.remove({ output: id });
  });
});
