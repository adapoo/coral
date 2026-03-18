import { C, F } from "@/lib/utils/general/colors";
import { findThreshold } from "@/lib/utils/general/threshold";

const XP_TO_NEXT_LEVEL = [
  0, 10, 25, 50, 75, 100, 250, 500, 750, 1000, 1250, 1500, 1750, 2000, 2500,
  3000, 3500, 4000, 4500,
];

const TOTAL_XP = XP_TO_NEXT_LEVEL.map((_, index) =>
  XP_TO_NEXT_LEVEL.slice(0, index + 1).reduce((acc, xp) => acc + xp, 0)
);

const CONSTANT_LEVELING_XP = XP_TO_NEXT_LEVEL.reduce((acc, xp) => acc + xp, 0);
const CONSTANT_XP_TO_NEXT_LEVEL = 5000;
const LEVEL_MAX = 500;

export const getSkywarsLevel = (xp: number): number => {
  if (xp >= CONSTANT_LEVELING_XP) {
    const level =
      Math.floor((xp - CONSTANT_LEVELING_XP) / CONSTANT_XP_TO_NEXT_LEVEL) +
      XP_TO_NEXT_LEVEL.length;

    return Math.min(level, LEVEL_MAX);
  }

  const level = TOTAL_XP.findIndex((x) => x > xp);
  return level;
};

const EMBLEM_MAP = {
  default: "✯",
  carrots_for_eyes: "^_^",
  formerly_known: "@_@",
  reflex_angle_eyebrows: "δvδ",
  two_tired: "zz_zz",
  slime: "■·■",
  same_great_taste: "ಠ_ಠ",
  misaligned: "o...0",
  converge_on_tongue: ">u<",
  no_evil: "v-v",
  three_fourths_jam: "༼つ◕_◕༽つ",
  alpha: "α",
  omega: "Ω",
  rich: "$",
  podium: "π",
  fallen_crest: "☬",
  null: "∅",
  sigma: "Σ",
  delta: "δ",
  florin: "ƒ",
};

type Scheme = (
  level: number,
  bold: boolean,
  underline: boolean,
  strikethrough: boolean,
  emblem?: string
) => string;

const SCHEME_MAP = {
  stone_prestige: createUniformScheme(C.GRAY),
  iron_prestige: createUniformScheme(C.WHITE),
  gold_prestige: createUniformScheme(C.GOLD),
  diamond_prestige: createUniformScheme(C.AQUA),
  ruby_prestige: createUniformScheme(C.RED),
  crystal_prestige: createUniformScheme(C.LIGHT_PURPLE),
  amethyst_prestige: createUniformScheme(C.DARK_PURPLE),
  opal_prestige: createUniformScheme(C.BLUE),
  topaz_prestige: createUniformScheme(C.YELLOW),
  jade_prestige: createUniformScheme(C.GREEN),
  mythic_i_prestige: createMultiDigitColorScheme([
    C.RED,
    C.GOLD,
    C.YELLOW,
    C.GREEN,
    C.AQUA,
    C.LIGHT_PURPLE,
  ]),
  bloody_prestige: createUniformScheme(C.DARK_RED, C.RED),
  cobalt_prestige: createUniformScheme(C.DARK_BLUE),
  content_prestige: createUniformScheme(C.RED, C.WHITE),
  crimson_prestige: createUniformScheme(C.DARK_RED),
  firefly_prestige: createUniformScheme(C.GOLD, C.YELLOW),
  emerald_prestige: createUniformScheme(C.DARK_GREEN),
  abyss_prestige: createUniformScheme(C.DARK_BLUE, C.BLUE),
  sapphire_prestige: createUniformScheme(C.DARK_AQUA),
  emergency_prestige: createUniformScheme(C.DARK_RED, C.YELLOW),
  mythic_ii_prestige: createMultiDigitColorScheme([
    C.GOLD,
    C.YELLOW,
    C.GREEN,
    C.AQUA,
    C.LIGHT_PURPLE,
    C.RED,
  ]),
  mulberry_prestige: createUniformScheme(C.DARK_PURPLE, C.LIGHT_PURPLE),
  slate_prestige: createUniformScheme(C.DARK_GRAY),
  blood_god_prestige: createUniformScheme(C.LIGHT_PURPLE, C.AQUA),
  midnight_prestige: createUniformScheme(C.BLACK),
  sun_prestige: createMultiDigitColorScheme([
    C.RED,
    C.GOLD,
    C.YELLOW,
    C.YELLOW,
    C.GOLD,
    C.RED,
  ]),
  bulb_prestige: createMultiDigitColorScheme([
    C.BLACK,
    C.YELLOW,
    C.GOLD,
    C.GOLD,
    C.YELLOW,
    C.BLACK,
  ]),
  twilight_prestige: createUniformScheme(C.DARK_BLUE, C.DARK_AQUA),
  natural_prestige: createMultiDigitColorScheme([
    C.GREEN,
    C.DARK_GREEN,
    C.GREEN,
    C.YELLOW,
    C.GREEN,
    C.DARK_GREEN,
  ]),
  icile_prestige: createUniformScheme(C.BLUE, C.AQUA),
  mythic_iii_prestige: createMultiDigitColorScheme([
    C.YELLOW,
    C.GREEN,
    C.AQUA,
    C.LIGHT_PURPLE,
    C.RED,
    C.GOLD,
  ]),
  graphite_prestige: createUniformScheme(C.DARK_GRAY, C.GRAY),
  punk_prestige: createUniformScheme(C.LIGHT_PURPLE, C.GREEN),
  meltdown_prestige: createUniformScheme(C.YELLOW, C.RED),
  iridescent_prestige: createMultiDigitColorScheme([
    C.AQUA,
    C.GREEN,
    C.AQUA,
    C.LIGHT_PURPLE,
    C.GREEN,
    C.GREEN,
  ]),
  marigold_prestige: createMultiDigitColorScheme([
    C.WHITE,
    C.WHITE,
    C.YELLOW,
    C.YELLOW,
    C.GOLD,
    C.GOLD,
  ]),
  beach_prestige: createMultiDigitColorScheme([
    C.BLUE,
    C.DARK_AQUA,
    C.AQUA,
    C.WHITE,
    C.YELLOW,
    C.YELLOW,
  ]),
  spark_prestige: createMultiDigitColorScheme([
    C.YELLOW,
    C.YELLOW,
    C.WHITE,
    C.WHITE,
    C.DARK_GRAY,
    C.DARK_GRAY,
  ]),
  target_prestige: createMultiDigitColorScheme([
    C.RED,
    C.WHITE,
    C.RED,
    C.RED,
    C.WHITE,
    C.RED,
  ]),
  limelight_prestige: createUniformScheme(C.DARK_GREEN, C.GREEN),
  mythic_iv_prestige: createMultiDigitColorScheme([
    C.GREEN,
    C.AQUA,
    C.LIGHT_PURPLE,
    C.RED,
    C.GOLD,
    C.YELLOW,
  ]),
  cerulean_prestige: createUniformScheme(C.DARK_AQUA, C.AQUA),
  magical_prestige: createMultiDigitColorScheme([
    C.BLACK,
    C.DARK_PURPLE,
    C.DARK_GRAY,
    C.DARK_GRAY,
    C.DARK_PURPLE,
    C.BLACK,
  ]),
  luminous_prestige: createMultiDigitColorScheme([
    C.GOLD,
    C.GOLD,
    C.WHITE,
    C.WHITE,
    C.AQUA,
    C.DARK_AQUA,
  ]),
  synthesis_prestige: createMultiDigitColorScheme([
    C.GREEN,
    C.DARK_GREEN,
    C.GREEN,
    C.YELLOW,
    C.WHITE,
    C.WHITE,
  ]),
  burn_prestige: createMultiDigitColorScheme([
    C.DARK_RED,
    C.DARK_RED,
    C.RED,
    C.GOLD,
    C.YELLOW,
    C.WHITE,
  ]),
  dramatic_prestige: createMultiDigitColorScheme([
    C.BLUE,
    C.AQUA,
    C.DARK_AQUA,
    C.LIGHT_PURPLE,
    C.DARK_PURPLE,
    C.DARK_RED,
  ]),
  radiant_prestige: createMultiDigitColorScheme([
    C.BLACK,
    C.DARK_GRAY,
    C.GRAY,
    C.WHITE,
    C.GRAY,
    C.DARK_GRAY,
  ]),
  tidal_prestige: createMultiDigitColorScheme([
    C.DARK_BLUE,
    C.DARK_BLUE,
    C.BLUE,
    C.DARK_AQUA,
    C.AQUA,
    C.WHITE,
  ]),
  firework_prestige: createMultiDigitColorScheme([
    C.BLUE,
    C.AQUA,
    C.WHITE,
    C.WHITE,
    C.RED,
    C.DARK_RED,
  ]),
  mythic_v_prestige: createMultiDigitColorScheme([
    C.AQUA,
    C.LIGHT_PURPLE,
    C.RED,
    C.GOLD,
    C.YELLOW,
    C.GREEN,
  ]),

  ancient: createUniformScheme(C.GRAY, C.DARK_GRAY),
  the_new_default: createUniformScheme(C.GOLD, C.GRAY, C.GOLD),
  the_new_new_default: createUniformScheme(C.AQUA, C.GRAY, C.AQUA),
  launch: createUniformScheme(C.GOLD, C.GOLD, C.DARK_GRAY),
  jersey: createUniformScheme(C.WHITE, C.WHITE, C.RED),
  spotlight: createUniformScheme(C.BLACK, C.WHITE),
  earth: createUniformScheme(C.DARK_RED, C.DARK_RED, C.GREEN),
  glint: createUniformScheme(C.LIGHT_PURPLE, C.LIGHT_PURPLE, C.AQUA),
  strength: createUniformScheme(C.RED, C.LIGHT_PURPLE),
  adrenaline: createUniformScheme(C.RED, C.GREEN),
  pumpkin: createUniformScheme(C.DARK_RED, C.GOLD),
  seashell: createUniformScheme(C.YELLOW, C.YELLOW, C.RED),
  obsidian: createUniformScheme(C.DARK_GRAY, C.DARK_GRAY, C.DARK_PURPLE),
  support: createUniformScheme(C.WHITE, C.RED),
  mahogany: createUniformScheme(C.YELLOW, C.GOLD),
  spell: createMultiDigitColorScheme([
    C.LIGHT_PURPLE,
    C.LIGHT_PURPLE,
    C.LIGHT_PURPLE,
    C.YELLOW,
    C.YELLOW,
    C.YELLOW,
  ]),
  pillar: createUniformScheme(C.WHITE, C.GOLD),
  agile: createUniformScheme(C.GREEN, C.WHITE),
  bone: createUniformScheme(C.WHITE, C.GRAY, C.WHITE),
  slimy: createUniformScheme(C.GREEN, C.DARK_GREEN),
  holiday: createUniformScheme(C.DARK_RED, C.GREEN),
  iconic: createUniformScheme(C.BLACK, C.BLACK, C.WHITE),
  // TODO: Figure out name: Level-conic?
  "level-conic?": createUniformScheme(C.BLACK, C.WHITE, C.BLACK),
  safari: createMultiDigitColorScheme([
    C.DARK_GREEN,
    C.DARK_GREEN,
    C.DARK_GREEN,
    C.GOLD,
    C.GOLD,
    C.GOLD,
  ]),
  gummy_worm: createMultiDigitColorScheme([
    C.RED,
    C.RED,
    C.RED,
    C.AQUA,
    C.AQUA,
    C.AQUA,
  ]),
  timetravel: createMultiDigitColorScheme([
    C.GRAY,
    C.BLACK,
    C.BLACK,
    C.GRAY,
    C.GRAY,
    C.GRAY,
  ]),
  horned: createUniformScheme(C.RED, C.DARK_GRAY),
  sandy: createMultiDigitColorScheme([
    C.GOLD,
    C.YELLOW,
    C.WHITE,
    C.YELLOW,
    C.GOLD,
    C.YELLOW,
  ]),
  brutus: createMultiDigitColorScheme([
    C.BLUE,
    C.BLUE,
    C.DARK_GRAY,
    C.DARK_GRAY,
    C.WHITE,
    C.WHITE,
  ]),
  coinsmith: createMultiDigitColorScheme([
    C.YELLOW,
    C.DARK_GRAY,
    C.DARK_GRAY,
    C.DARK_GRAY,
    C.GOLD,
    C.YELLOW,
  ]),
  soulsmith: createMultiDigitColorScheme([
    C.GRAY,
    C.AQUA,
    C.AQUA,
    C.WHITE,
    C.WHITE,
    C.WHITE,
  ]),
  grand_slam: createUniformScheme(C.DARK_GREEN, C.GREEN, C.WHITE),
  fleet: createMultiDigitColorScheme([
    C.BLACK,
    C.RED,
    C.YELLOW,
    C.GREEN,
    C.GREEN,
    C.BLACK,
  ]),
  vengeance: createUniformScheme(C.BLACK, C.DARK_GRAY, C.YELLOW),
  dry: createUniformScheme(C.YELLOW, C.WHITE, C.GOLD),
  prickly: createUniformScheme(C.YELLOW, C.GREEN, C.WHITE),
  cast_iron: createMultiDigitColorScheme([
    C.GRAY,
    C.GRAY,
    C.DARK_GRAY,
    C.DARK_GRAY,
    C.DARK_AQUA,
    C.DARK_AQUA,
  ]),
  explosive: createMultiDigitColorScheme([
    C.RED,
    C.RED,
    C.YELLOW,
    C.YELLOW,
    C.GOLD,
    C.GOLD,
  ]),
  verdant: createMultiDigitColorScheme([
    C.DARK_GREEN,
    C.GREEN,
    C.GREEN,
    C.YELLOW,
    C.GOLD,
    C.YELLOW,
  ]),
  enchantment: createMultiDigitColorScheme([
    C.WHITE,
    C.LIGHT_PURPLE,
    C.DARK_PURPLE,
    C.DARK_PURPLE,
    C.LIGHT_PURPLE,
    C.WHITE,
  ]),
  void: createUniformScheme(C.DARK_GRAY, C.DARK_PURPLE, C.LIGHT_PURPLE),
  fragile: createUniformScheme(C.BLACK, C.DARK_AQUA, C.GREEN),
  mite: createMultiDigitColorScheme([
    C.DARK_AQUA,
    C.DARK_GREEN,
    C.DARK_GRAY,
    C.DARK_GREEN,
    C.GREEN,
    C.DARK_AQUA,
  ]),
  shulker: createUniformScheme(C.DARK_PURPLE, C.YELLOW, C.WHITE),
  redstone: createUniformScheme(C.BLACK, C.RED, C.DARK_RED),
  technical: createMultiDigitColorScheme([
    C.RED,
    C.RED,
    C.GRAY,
    C.GRAY,
    C.DARK_GRAY,
    C.DARK_GRAY,
  ]),
  melon: createMultiDigitColorScheme([
    C.GREEN,
    C.DARK_GREEN,
    C.GREEN,
    C.DARK_GREEN,
    C.YELLOW,
    C.GREEN,
  ]),
  driftwood: createMultiDigitColorScheme([
    C.DARK_AQUA,
    C.DARK_AQUA,
    C.YELLOW,
    C.YELLOW,
    C.DARK_RED,
    C.DARK_RED,
  ]),
  river: createUniformScheme(C.DARK_GREEN, C.BLUE, C.GREEN),
  mangrove: createMultiDigitColorScheme([
    C.DARK_RED,
    C.DARK_RED,
    C.RED,
    C.RED,
    C.DARK_GREEN,
    C.DARK_GREEN,
  ]),
  jeremiah: createUniformScheme(C.DARK_AQUA, C.GOLD, C.YELLOW),
  poppy: createMultiDigitColorScheme([
    C.RED,
    C.DARK_RED,
    C.BLACK,
    C.BLACK,
    C.DARK_RED,
    C.RED,
  ]),
  creeper: createMultiDigitColorScheme([
    C.WHITE,
    C.WHITE,
    C.GREEN,
    C.GREEN,
    C.DARK_GREEN,
    C.DARK_GREEN,
  ]),
  camo: createMultiDigitColorScheme([
    C.DARK_GRAY,
    C.DARK_GRAY,
    C.DARK_GREEN,
    C.DARK_GREEN,
    C.GREEN,
    C.GREEN,
  ]),
  first_aid: createUniformScheme(C.DARK_RED, C.WHITE, C.RED),
  penguin: createUniformScheme(C.DARK_GRAY, C.BLUE, C.YELLOW),
  nether: createMultiDigitColorScheme([
    C.GRAY,
    C.GRAY,
    C.DARK_AQUA,
    C.DARK_AQUA,
    C.RED,
    C.RED,
  ]),
  wilderness: createMultiDigitColorScheme([
    C.DARK_GREEN,
    C.DARK_GREEN,
    C.DARK_AQUA,
    C.DARK_AQUA,
    C.GOLD,
    C.GOLD,
  ]),
  one_stone: createMultiDigitColorScheme([
    C.GRAY,
    C.GRAY,
    C.DARK_GREEN,
    C.DARK_GREEN,
    C.DARK_GRAY,
    C.DARK_GRAY,
  ]),
  circus: createMultiDigitColorScheme([
    C.RED,
    C.RED,
    C.GOLD,
    C.GOLD,
    C.DARK_GREEN,
    C.DARK_GREEN,
  ]),
  veracious: createUniformScheme(C.DARK_PURPLE, C.WHITE, C.GOLD),
  valiant: createUniformScheme(C.RED, C.WHITE, C.GREEN),
  venerable: createUniformScheme(C.BLUE, C.WHITE, C.YELLOW),
  portal: createMultiDigitColorScheme([
    C.GREEN,
    C.GREEN,
    C.LIGHT_PURPLE,
    C.LIGHT_PURPLE,
    C.RED,
    C.RED,
  ]),
  sorcratic: createUniformScheme(C.DARK_GRAY, C.WHITE, C.YELLOW),
  parallel_dimension: createMultiDigitColorScheme([
    C.BLUE,
    C.BLUE,
    C.DARK_GRAY,
    C.DARK_GRAY,
    C.LIGHT_PURPLE,
    C.LIGHT_PURPLE,
  ]),
  tomb: createMultiDigitColorScheme([
    C.GOLD,
    C.BLUE,
    C.GOLD,
    C.BLUE,
    C.YELLOW,
    C.YELLOW,
  ]),
  irigation: createMultiDigitColorScheme([
    C.AQUA,
    C.AQUA,
    C.GREEN,
    C.GOLD,
    C.YELLOW,
    C.YELLOW,
  ]),
  snout: createMultiDigitColorScheme([
    C.DARK_PURPLE,
    C.BLACK,
    C.LIGHT_PURPLE,
    C.LIGHT_PURPLE,
    C.BLACK,
    C.DARK_PURPLE,
  ]),
  potato: createMultiDigitColorScheme([
    C.YELLOW,
    C.LIGHT_PURPLE,
    C.LIGHT_PURPLE,
    C.RED,
    C.RED,
    C.DARK_GRAY,
  ]),
  royal: createMultiDigitColorScheme([
    C.BLUE,
    C.BLUE,
    C.GOLD,
    C.GOLD,
    C.RED,
    C.RED,
  ]),
  bubblegum: createMultiDigitColorScheme([
    C.DARK_PURPLE,
    C.LIGHT_PURPLE,
    C.LIGHT_PURPLE,
    C.WHITE,
    C.WHITE,
    C.LIGHT_PURPLE,
  ]),
  insane: createUniformScheme(C.GRAY, C.WHITE, C.GOLD),
  smoke: createMultiDigitColorScheme([
    C.BLACK,
    C.BLACK,
    C.DARK_GRAY,
    C.DARK_GRAY,
    C.WHITE,
    C.WHITE,
  ]),
  scarlet: createMultiDigitColorScheme([
    C.DARK_GRAY,
    C.DARK_GRAY,
    C.DARK_RED,
    C.DARK_RED,
    C.RED,
    C.RED,
  ]),
  afterburn: createMultiDigitColorScheme([
    C.AQUA,
    C.AQUA,
    C.GOLD,
    C.DARK_GRAY,
    C.DARK_GRAY,
    C.GRAY,
  ]),
  normal: createUniformScheme(C.DARK_GRAY, C.GRAY, C.GOLD),
  salmon: createMultiDigitColorScheme([
    C.RED,
    C.RED,
    C.DARK_AQUA,
    C.DARK_AQUA,
    C.DARK_GREEN,
    C.DARK_GREEN,
  ]),
  lucky: createUniformScheme(C.BLACK, C.DARK_GREEN, C.GOLD),
  likeable: createMultiDigitColorScheme([
    C.DARK_RED,
    C.DARK_RED,
    C.RED,
    C.RED,
    C.WHITE,
    C.WHITE,
  ]),
  lunar: createMultiDigitColorScheme([
    C.WHITE,
    C.WHITE,
    C.WHITE,
    C.GRAY,
    C.DARK_GRAY,
    C.DARK_GRAY,
  ]),
  hypixel: createUniformScheme(C.DARK_RED, C.GOLD, C.YELLOW),
  sky: createMultiDigitColorScheme([
    C.YELLOW,
    C.YELLOW,
    C.AQUA,
    C.AQUA,
    C.WHITE,
    C.WHITE,
  ]),
  frosty: createUniformScheme(C.DARK_GRAY, C.WHITE, C.GRAY),
  treasure: createMultiDigitColorScheme([
    C.GOLD,
    C.GOLD,
    C.WHITE,
    C.WHITE,
    C.YELLOW,
    C.YELLOW,
  ]),
  gemstone: createMultiDigitColorScheme([
    C.DARK_RED,
    C.RED,
    C.WHITE,
    C.WHITE,
    C.RED,
    C.DARK_RED,
  ]),
  dark_magic: createMultiDigitColorScheme([
    C.DARK_RED,
    C.DARK_RED,
    C.DARK_PURPLE,
    C.DARK_PURPLE,
    C.RED,
    C.RED,
  ]),
  reflections: createMultiDigitColorScheme([
    C.DARK_BLUE,
    C.BLACK,
    C.BLACK,
    C.LIGHT_PURPLE,
    C.LIGHT_PURPLE,
    C.DARK_PURPLE,
  ]),
  brewery: createUniformScheme(C.DARK_PURPLE, C.RED, `${C.LIGHT_PURPLE}$`),
  leo: createMultiDigitColorScheme([
    C.YELLOW,
    C.YELLOW,
    C.YELLOW,
    C.GOLD,
    C.DARK_RED,
    C.DARK_RED,
  ]),
  zebra: createMultiDigitColorScheme([
    C.GRAY,
    C.DARK_GRAY,
    C.GRAY,
    C.DARK_GRAY,
    C.WHITE,
    C.DARK_GRAY,
  ]),
  emit: createMultiDigitColorScheme([
    C.DARK_PURPLE,
    C.LIGHT_PURPLE,
    C.WHITE,
    C.WHITE,
    C.LIGHT_PURPLE,
    C.DARK_PURPLE,
  ]),
  smoldering: createUniformScheme(C.BLACK, C.DARK_RED, C.RED),
  stormy: createMultiDigitColorScheme([
    C.YELLOW,
    C.YELLOW,
    C.WHITE,
    C.WHITE,
    C.GRAY,
    C.GRAY,
  ]),
  borealis: createMultiDigitColorScheme([
    C.LIGHT_PURPLE,
    C.LIGHT_PURPLE,
    C.AQUA,
    C.AQUA,
    C.GREEN,
    C.GREEN,
  ]),
  devil: createMultiDigitColorScheme([
    C.BLACK,
    C.DARK_GRAY,
    C.DARK_GRAY,
    C.DARK_RED,
    C.DARK_RED,
    C.RED,
  ]),
  demigod: createMultiDigitColorScheme(
    [C.DARK_GRAY, C.GOLD, C.YELLOW, C.GRAY, C.DARK_GRAY, C.DARK_GRAY],
    "curly"
  ),
  laurel: createMultiDigitColorScheme([
    C.DARK_GREEN,
    C.DARK_GREEN,
    C.GOLD,
    C.GOLD,
    C.WHITE,
    C.WHITE,
  ]),
  uplifting: createMultiDigitColorScheme([
    C.DARK_GRAY,
    C.DARK_GRAY,
    C.GRAY,
    C.GRAY,
    C.YELLOW,
    C.YELLOW,
  ]),
  the_world_moves_on: createMultiDigitColorScheme([
    C.DARK_GRAY,
    C.DARK_GRAY,
    C.GOLD,
    C.GOLD,
    C.RED,
    C.RED,
  ]),
  swine: createMultiDigitColorScheme([
    C.DARK_PURPLE,
    C.DARK_PURPLE,
    C.LIGHT_PURPLE,
    C.LIGHT_PURPLE,
    C.WHITE,
    C.WHITE,
  ]),
  beagle: createUniformScheme(C.WHITE, C.GRAY, C.WHITE),
  the_prestige_prestige: createMultiDigitColorScheme([
    C.GRAY,
    C.WHITE,
    C.GOLD,
    C.AQUA,
    C.RED,
    C.LIGHT_PURPLE,
  ]),
  opalsmith: createMultiDigitColorScheme([
    C.BLUE,
    C.BLUE,
    C.AQUA,
    C.DARK_AQUA,
    C.LIGHT_PURPLE,
    C.DARK_PURPLE,
  ]),
  scurvy: createMultiDigitColorScheme([
    C.BLUE,
    C.DARK_AQUA,
    C.AQUA,
    C.WHITE,
    C.GREEN,
    C.DARK_GREEN,
  ]),
  fools_mythic: createMultiDigitColorScheme([
    C.DARK_RED,
    C.RED,
    C.GOLD,
    C.DARK_GREEN,
    C.BLUE,
    C.DARK_PURPLE,
  ]),
  eponymous: createMultiDigitColorScheme([
    C.DARK_AQUA,
    C.DARK_AQUA,
    C.DARK_GREEN,
    C.GREEN,
    C.YELLOW,
    C.GOLD,
  ]),
  bandage: createMultiDigitColorScheme([
    C.BLACK,
    C.DARK_GRAY,
    C.GRAY,
    C.WHITE,
    C.RED,
    C.DARK_RED,
  ]),
  clown: createMultiDigitColorScheme([
    C.DARK_GREEN,
    C.RED,
    C.WHITE,
    C.WHITE,
    C.RED,
    C.DARK_RED,
  ]),
  tropical: createMultiDigitColorScheme([
    C.YELLOW,
    C.BLUE,
    C.GOLD,
    C.DARK_AQUA,
    C.RED,
    C.DARK_BLUE,
  ]),
  sugar_crash: createMultiDigitColorScheme([
    C.WHITE,
    C.YELLOW,
    C.RED,
    C.LIGHT_PURPLE,
    C.AQUA,
    C.WHITE,
  ]),
  ultraviolence: createMultiDigitColorScheme([
    C.DARK_GREEN,
    C.GREEN,
    C.WHITE,
    C.WHITE,
    C.LIGHT_PURPLE,
    C.DARK_PURPLE,
  ]),
} satisfies Record<string, Scheme>;

/**
 * Formats color schemes when at most the bracket, digit, and emblem color differ
 */
function createUniformScheme(
  bracketColor: string,
  digitColor = bracketColor,
  emblemColor = digitColor
): Scheme {
  return (level, bold, underline, strikethrough, emblem) => {
    const boldFormat = bold ? F.BOLD : "";
    const underlineFormat = underline ? F.UNDERLINE : "";
    const strikethroughFormat = strikethrough ? F.STRIKETHROUGH : "";

    return `${bracketColor}${underlineFormat}${strikethroughFormat}[${F.RESET}${boldFormat}${digitColor}${underlineFormat}${underlineFormat}${level}${emblemColor}${underlineFormat}${emblem}${F.RESET}${bracketColor}${underlineFormat}${strikethroughFormat}]${F.RESET}`;
  };
}

/**
 * Formats color schemes where almost every digit has a different color
 */
function createMultiDigitColorScheme(
  colors: [
    leftBracket: string,
    firstDigit: string,
    secondDigit: string,
    thirdDigit: string,
    emblem: string,
    rightBracket: string
  ],
  bracketKind: "square" | "curly" = "square"
): Scheme {
  const leftBracket = bracketKind === "square" ? "[" : "{";
  const rightBracket = bracketKind === "square" ? "]" : "}";

  return (level, bold, underline, strikethrough, emblem) => {
    const boldFormat = bold ? F.BOLD : "";
    const underlineFormat = underline ? F.UNDERLINE : "";
    const strikethroughFormat = strikethrough ? F.STRIKETHROUGH : "";

    const formattedColors = colors.map(
      (color) => `${color}${underlineFormat}`
    ) as [
      leftBracket: string,
      firstDigit: string,
      secondDigit: string,
      thirdDigit: string,
      emblem: string,
      rightBracket: string
    ];

    const formattedEmblem = emblem ? `${formattedColors.at(-2)}${emblem}` : "";
    const formattedLevel = [...`${level}`]
      .toReversed()
      .map((digit, index) => `${formattedColors[3 - index]}${digit}`)
      .toReversed()
      .join("");

    return `${F.RESET}${
      formattedColors[0]
    }${strikethroughFormat}${leftBracket}${
      F.RESET
    }${boldFormat}${formattedLevel}${formattedEmblem}${
      F.RESET
    }${formattedColors.at(-1)}${strikethroughFormat}${rightBracket}${F.RESET}`;
  };
}

const PRESTIGE_SCHEMES: { req: number; scheme: keyof typeof SCHEME_MAP }[] = [
  { req: 0, scheme: "stone_prestige" },
  { req: 10, scheme: "iron_prestige" },
  { req: 20, scheme: "gold_prestige" },
  { req: 30, scheme: "diamond_prestige" },
  { req: 40, scheme: "ruby_prestige" },
  { req: 50, scheme: "crystal_prestige" },
  { req: 60, scheme: "amethyst_prestige" },
  { req: 70, scheme: "opal_prestige" },
  { req: 80, scheme: "topaz_prestige" },
  { req: 90, scheme: "jade_prestige" },
  { req: 100, scheme: "mythic_i_prestige" },
  { req: 110, scheme: "bloody_prestige" },
  { req: 120, scheme: "cobalt_prestige" },
  { req: 130, scheme: "content_prestige" },
  { req: 140, scheme: "crimson_prestige" },
  { req: 150, scheme: "firefly_prestige" },
  { req: 160, scheme: "emerald_prestige" },
  { req: 170, scheme: "abyss_prestige" },
  { req: 180, scheme: "sapphire_prestige" },
  { req: 190, scheme: "emergency_prestige" },
  { req: 200, scheme: "mythic_ii_prestige" },
  { req: 210, scheme: "mulberry_prestige" },
  { req: 220, scheme: "slate_prestige" },
  { req: 230, scheme: "blood_god_prestige" },
  { req: 240, scheme: "midnight_prestige" },
  { req: 250, scheme: "sun_prestige" },
  { req: 260, scheme: "bulb_prestige" },
  { req: 270, scheme: "twilight_prestige" },
  { req: 280, scheme: "natural_prestige" },
  { req: 290, scheme: "icile_prestige" },
  { req: 300, scheme: "mythic_iii_prestige" },
  { req: 310, scheme: "graphite_prestige" },
  { req: 320, scheme: "punk_prestige" },
  { req: 330, scheme: "meltdown_prestige" },
  { req: 340, scheme: "iridescent_prestige" },
  { req: 350, scheme: "marigold_prestige" },
  { req: 360, scheme: "beach_prestige" },
  { req: 370, scheme: "spark_prestige" },
  { req: 380, scheme: "target_prestige" },
  { req: 390, scheme: "limelight_prestige" },
  { req: 400, scheme: "mythic_iv_prestige" },
  { req: 410, scheme: "cerulean_prestige" },
  { req: 420, scheme: "magical_prestige" },
  { req: 430, scheme: "luminous_prestige" },
  { req: 440, scheme: "synthesis_prestige" },
  { req: 450, scheme: "burn_prestige" },
  { req: 460, scheme: "dramatic_prestige" },
  { req: 470, scheme: "radiant_prestige" },
  { req: 480, scheme: "tidal_prestige" },
  { req: 490, scheme: "firework_prestige" },
  { req: 500, scheme: "mythic_v_prestige" },
];

const PRESTIGE_EMBLEMS: { req: number; emblem: keyof typeof EMBLEM_MAP }[] = [
  { req: 0, emblem: "default" },
  { req: 50, emblem: "carrots_for_eyes" },
  { req: 100, emblem: "formerly_known" },
  { req: 150, emblem: "reflex_angle_eyebrows" },
  { req: 200, emblem: "two_tired" },
  { req: 250, emblem: "slime" },
  { req: 300, emblem: "same_great_taste" },
  { req: 350, emblem: "misaligned" },
  { req: 400, emblem: "converge_on_tongue" },
  { req: 450, emblem: "no_evil" },
  { req: 500, emblem: "three_fourths_jam" },
];

const BOLD_LEVEL_REQUIREMENT = 300;
const UNDERLINE_LEVEL_REQUIREMENT = 400;
const STRIKETHROUGH_LEVEL_REQUIREMENT = 500;

/**
 * Gets a player's formatted level based on what scheme and emblem they should have access to at their level
 */
export const formatSkywarsLevelIntended = (level: number) => {
  level = Math.floor(level);

  const { emblem: emblemKey } = findThreshold(PRESTIGE_EMBLEMS, level);
  const { scheme: schemeKey } = findThreshold(PRESTIGE_SCHEMES, level);

  const emblem = EMBLEM_MAP[emblemKey];
  const scheme = SCHEME_MAP[schemeKey];

  return scheme(
    level,
    level >= BOLD_LEVEL_REQUIREMENT,
    level >= UNDERLINE_LEVEL_REQUIREMENT,
    level >= STRIKETHROUGH_LEVEL_REQUIREMENT,
    emblem
  );
};

/**
 * Gets a player's formatted level based on their preferences
 */
export function formatSkywarsLevel(
  level: number,
  selectedScheme: string | undefined,
  selectedEmblem: string | undefined,
  bold: boolean,
  underline: boolean,
  strikethrough: boolean
) {
  selectedScheme = selectedScheme?.replace("scheme_", "");
  selectedEmblem = selectedEmblem?.replace("emblem_", "");

  let schemeKey: keyof typeof SCHEME_MAP;
  let emblemKey: keyof typeof EMBLEM_MAP | undefined = undefined;

  if (selectedScheme) {
    if (selectedScheme in SCHEME_MAP) {
      schemeKey = selectedScheme as keyof typeof SCHEME_MAP;
    } else {
      schemeKey = findThreshold(PRESTIGE_SCHEMES, level).scheme;
    }
  } else {
    schemeKey = findThreshold(PRESTIGE_SCHEMES, level).scheme;
  }

  if (selectedEmblem) {
    if (selectedEmblem in EMBLEM_MAP) {
      emblemKey = selectedEmblem as keyof typeof EMBLEM_MAP;
    } else {
      emblemKey = findThreshold(PRESTIGE_EMBLEMS, level).emblem;
    }
  } else {
    emblemKey = "default";
  }

  const emblem = emblemKey ? EMBLEM_MAP[emblemKey] : undefined;
  const scheme = SCHEME_MAP[schemeKey];

  return scheme(level, bold, underline, strikethrough, emblem);
}

const MYTHICAL_KIT = "kit_mythical_";
const TEAMS = "team_";
const SOLO = "solo_";

const removeAllBeforePrefix = (str: string, prefix: string) => {
  const lastIndex = str.lastIndexOf(prefix);
  if (lastIndex === -1) return str;
  return str.slice(Math.max(0, lastIndex + prefix.length));
};

export const parseKit = (kit = "default") => {
  const parsedSolo = removeAllBeforePrefix(kit, SOLO);
  const parsedTeam = removeAllBeforePrefix(parsedSolo, TEAMS);
  return parsedTeam.replace(MYTHICAL_KIT, "").replaceAll("-", "_");
};
