// This is part of Jikan
// See README.md for details

use chrono::{NaiveDate, Datelike};
use std::collections::HashMap;

pub fn is_kanji(character: char) -> bool {
    match character {
        '\u{4e00}'...'\u{9faf}' |
        '\u{3400}'...'\u{4dbf}' => true,
        _ => false,
    }
}

lazy_static! {
    static ref LUNAR_NEW_YEAR: HashMap<u32, NaiveDate> = {
        let mut m = HashMap::new();
        m.insert(645, NaiveDate::from_ymd(645, 2, 5));
        m.insert(646, NaiveDate::from_ymd(646, 1, 25));
        m.insert(647, NaiveDate::from_ymd(647, 2, 13));
        m.insert(648, NaiveDate::from_ymd(648, 2, 2));
        m.insert(649, NaiveDate::from_ymd(649, 2, 20));
        m.insert(650, NaiveDate::from_ymd(650, 2, 10));
        m.insert(651, NaiveDate::from_ymd(651, 1, 30));
        m.insert(652, NaiveDate::from_ymd(652, 2, 18));
        m.insert(653, NaiveDate::from_ymd(653, 2, 6));
        m.insert(654, NaiveDate::from_ymd(654, 1, 27));
        m.insert(655, NaiveDate::from_ymd(655, 2, 15));
        m.insert(686, NaiveDate::from_ymd(686, 2, 2));
        m.insert(701, NaiveDate::from_ymd(701, 2, 17));
        m.insert(702, NaiveDate::from_ymd(702, 2, 6));
        m.insert(703, NaiveDate::from_ymd(703, 1, 26));
        m.insert(704, NaiveDate::from_ymd(704, 2, 14));
        m.insert(705, NaiveDate::from_ymd(705, 2, 3));
        m.insert(706, NaiveDate::from_ymd(706, 1, 23));
        m.insert(707, NaiveDate::from_ymd(707, 2, 11));
        m.insert(708, NaiveDate::from_ymd(708, 2, 1));
        m.insert(709, NaiveDate::from_ymd(709, 2, 18));
        m.insert(710, NaiveDate::from_ymd(710, 2, 7));
        m.insert(711, NaiveDate::from_ymd(711, 1, 27));
        m.insert(712, NaiveDate::from_ymd(712, 2, 15));
        m.insert(713, NaiveDate::from_ymd(713, 2, 4));
        m.insert(714, NaiveDate::from_ymd(714, 1, 25));
        m.insert(715, NaiveDate::from_ymd(715, 2, 13));
        m.insert(716, NaiveDate::from_ymd(716, 2, 1));
        m.insert(717, NaiveDate::from_ymd(717, 2, 20));
        m.insert(718, NaiveDate::from_ymd(718, 2, 9));
        m.insert(719, NaiveDate::from_ymd(719, 1, 29));
        m.insert(720, NaiveDate::from_ymd(720, 2, 17));
        m.insert(721, NaiveDate::from_ymd(721, 2, 5));
        m.insert(722, NaiveDate::from_ymd(722, 1, 26));
        m.insert(723, NaiveDate::from_ymd(723, 2, 14));
        m.insert(724, NaiveDate::from_ymd(724, 2, 4));
        m.insert(725, NaiveDate::from_ymd(725, 1, 23));
        m.insert(726, NaiveDate::from_ymd(726, 2, 11));
        m.insert(727, NaiveDate::from_ymd(727, 2, 1));
        m.insert(728, NaiveDate::from_ymd(728, 2, 19));
        m.insert(729, NaiveDate::from_ymd(729, 2, 7));
        m.insert(730, NaiveDate::from_ymd(730, 1, 27));
        m.insert(731, NaiveDate::from_ymd(731, 2, 15));
        m.insert(732, NaiveDate::from_ymd(732, 2, 5));
        m.insert(733, NaiveDate::from_ymd(733, 1, 25));
        m.insert(734, NaiveDate::from_ymd(734, 2, 12));
        m.insert(735, NaiveDate::from_ymd(735, 1, 29));
        m.insert(736, NaiveDate::from_ymd(736, 2, 20));
        m.insert(737, NaiveDate::from_ymd(737, 2, 8));
        m.insert(738, NaiveDate::from_ymd(738, 1, 29));
        m.insert(739, NaiveDate::from_ymd(739, 2, 17));
        m.insert(740, NaiveDate::from_ymd(740, 2, 6));
        m.insert(741, NaiveDate::from_ymd(741, 1, 26));
        m.insert(742, NaiveDate::from_ymd(742, 2, 14));
        m.insert(743, NaiveDate::from_ymd(743, 2, 2));
        m.insert(744, NaiveDate::from_ymd(744, 1, 24));
        m.insert(745, NaiveDate::from_ymd(745, 2, 10));
        m.insert(746, NaiveDate::from_ymd(746, 1, 30));
        m.insert(747, NaiveDate::from_ymd(747, 2, 18));
        m.insert(748, NaiveDate::from_ymd(748, 2, 8));
        m.insert(749, NaiveDate::from_ymd(749, 1, 27));
        m.insert(750, NaiveDate::from_ymd(750, 2, 15));
        m.insert(751, NaiveDate::from_ymd(751, 2, 5));
        m.insert(752, NaiveDate::from_ymd(752, 1, 25));
        m.insert(753, NaiveDate::from_ymd(753, 2, 12));
        m.insert(754, NaiveDate::from_ymd(754, 2, 1));
        m.insert(755, NaiveDate::from_ymd(755, 2, 20));
        m.insert(756, NaiveDate::from_ymd(756, 2, 9));
        m.insert(757, NaiveDate::from_ymd(757, 1, 29));
        m.insert(758, NaiveDate::from_ymd(758, 2, 17));
        m.insert(759, NaiveDate::from_ymd(759, 2, 6));
        m.insert(760, NaiveDate::from_ymd(760, 1, 27));
        m.insert(761, NaiveDate::from_ymd(761, 2, 14));
        m.insert(762, NaiveDate::from_ymd(762, 2, 1));
        m.insert(763, NaiveDate::from_ymd(763, 2, 21));
        m.insert(764, NaiveDate::from_ymd(764, 2, 11));
        m.insert(765, NaiveDate::from_ymd(765, 1, 30));
        m.insert(766, NaiveDate::from_ymd(766, 2, 18));
        m.insert(767, NaiveDate::from_ymd(767, 2, 8));
        m.insert(768, NaiveDate::from_ymd(768, 1, 28));
        m.insert(769, NaiveDate::from_ymd(769, 2, 15));
        m.insert(770, NaiveDate::from_ymd(770, 2, 4));
        m.insert(771, NaiveDate::from_ymd(771, 1, 25));
        m.insert(772, NaiveDate::from_ymd(772, 2, 12));
        m.insert(773, NaiveDate::from_ymd(773, 2, 1));
        m.insert(774, NaiveDate::from_ymd(774, 2, 20));
        m.insert(775, NaiveDate::from_ymd(775, 2, 9));
        m.insert(776, NaiveDate::from_ymd(776, 1, 20));
        m.insert(777, NaiveDate::from_ymd(777, 2, 17));
        m.insert(778, NaiveDate::from_ymd(778, 2, 6));
        m.insert(779, NaiveDate::from_ymd(779, 1, 26));
        m.insert(780, NaiveDate::from_ymd(780, 2, 15));
        m.insert(781, NaiveDate::from_ymd(781, 2, 3));
        m.insert(782, NaiveDate::from_ymd(782, 1, 22));
        m.insert(783, NaiveDate::from_ymd(783, 2, 10));
        m.insert(784, NaiveDate::from_ymd(784, 1, 31));
        m.insert(785, NaiveDate::from_ymd(785, 2, 18));
        m.insert(786, NaiveDate::from_ymd(786, 2, 8));
        m.insert(787, NaiveDate::from_ymd(787, 1, 28));
        m.insert(788, NaiveDate::from_ymd(788, 2, 16));
        m.insert(789, NaiveDate::from_ymd(789, 2, 4));
        m.insert(790, NaiveDate::from_ymd(790, 1, 24));
        m.insert(791, NaiveDate::from_ymd(791, 2, 12));
        m.insert(792, NaiveDate::from_ymd(792, 2, 1));
        m.insert(793, NaiveDate::from_ymd(793, 2, 19));
        m.insert(794, NaiveDate::from_ymd(794, 2, 9));
        m.insert(795, NaiveDate::from_ymd(795, 1, 30));
        m.insert(796, NaiveDate::from_ymd(796, 2, 18));
        m.insert(797, NaiveDate::from_ymd(797, 2, 6));
        m.insert(798, NaiveDate::from_ymd(798, 1, 26));
        m.insert(799, NaiveDate::from_ymd(799, 2, 14));
        m.insert(800, NaiveDate::from_ymd(800, 2, 3));
        m.insert(801, NaiveDate::from_ymd(801, 1, 22));
        m.insert(802, NaiveDate::from_ymd(802, 2, 10));
        m.insert(803, NaiveDate::from_ymd(803, 1, 31));
        m.insert(804, NaiveDate::from_ymd(804, 2, 19));
        m.insert(805, NaiveDate::from_ymd(805, 2, 7));
        m.insert(806, NaiveDate::from_ymd(806, 1, 28));
        m.insert(807, NaiveDate::from_ymd(807, 2, 16));
        m.insert(808, NaiveDate::from_ymd(808, 2, 4));
        m.insert(809, NaiveDate::from_ymd(809, 1, 24));
        m.insert(810, NaiveDate::from_ymd(810, 2, 12));
        m.insert(811, NaiveDate::from_ymd(811, 2, 1));
        m.insert(812, NaiveDate::from_ymd(812, 2, 20));
        m.insert(813, NaiveDate::from_ymd(813, 2, 9));
        m.insert(814, NaiveDate::from_ymd(814, 1, 29));
        m.insert(815, NaiveDate::from_ymd(815, 2, 17));
        m.insert(816, NaiveDate::from_ymd(816, 2, 6));
        m.insert(817, NaiveDate::from_ymd(817, 1, 25));
        m.insert(818, NaiveDate::from_ymd(818, 2, 13));
        m.insert(819, NaiveDate::from_ymd(819, 2, 3));
        m.insert(820, NaiveDate::from_ymd(820, 1, 23));
        m.insert(821, NaiveDate::from_ymd(821, 2, 10));
        m.insert(822, NaiveDate::from_ymd(822, 1, 31));
        m.insert(823, NaiveDate::from_ymd(823, 2, 19));
        m.insert(824, NaiveDate::from_ymd(824, 2, 6));
        m.insert(825, NaiveDate::from_ymd(825, 1, 27));
        m.insert(826, NaiveDate::from_ymd(826, 2, 14));
        m.insert(827, NaiveDate::from_ymd(827, 2, 4));
        m.insert(828, NaiveDate::from_ymd(828, 1, 25));
        m.insert(829, NaiveDate::from_ymd(829, 2, 12));
        m.insert(830, NaiveDate::from_ymd(830, 2, 1));
        m.insert(831, NaiveDate::from_ymd(831, 2, 20));
        m.insert(832, NaiveDate::from_ymd(832, 2, 10));
        m.insert(833, NaiveDate::from_ymd(833, 1, 29));
        m.insert(834, NaiveDate::from_ymd(834, 2, 16));
        m.insert(835, NaiveDate::from_ymd(835, 2, 6));
        m.insert(836, NaiveDate::from_ymd(836, 1, 26));
        m.insert(837, NaiveDate::from_ymd(837, 2, 13));
        m.insert(838, NaiveDate::from_ymd(838, 2, 2));
        m.insert(839, NaiveDate::from_ymd(839, 1, 23));
        m.insert(840, NaiveDate::from_ymd(840, 2, 11));
        m.insert(841, NaiveDate::from_ymd(841, 1, 30));
        m.insert(842, NaiveDate::from_ymd(842, 2, 18));
        m.insert(843, NaiveDate::from_ymd(843, 2, 7));
        m.insert(844, NaiveDate::from_ymd(844, 1, 27));
        m.insert(845, NaiveDate::from_ymd(845, 2, 14));
        m.insert(846, NaiveDate::from_ymd(846, 2, 4));
        m.insert(847, NaiveDate::from_ymd(847, 1, 25));
        m.insert(848, NaiveDate::from_ymd(848, 2, 13));
        m.insert(849, NaiveDate::from_ymd(849, 2, 1));
        m.insert(850, NaiveDate::from_ymd(850, 2, 20));
        m.insert(851, NaiveDate::from_ymd(851, 2, 9));
        m.insert(852, NaiveDate::from_ymd(852, 1, 29));
        m.insert(853, NaiveDate::from_ymd(853, 2, 16));
        m.insert(854, NaiveDate::from_ymd(854, 2, 5));
        m.insert(855, NaiveDate::from_ymd(855, 1, 27));
        m.insert(856, NaiveDate::from_ymd(856, 2, 14));
        m.insert(857, NaiveDate::from_ymd(857, 2, 3));
        m.insert(858, NaiveDate::from_ymd(858, 1, 23));
        m.insert(859, NaiveDate::from_ymd(859, 2, 11));
        m.insert(860, NaiveDate::from_ymd(860, 1, 31));
        m.insert(861, NaiveDate::from_ymd(861, 2, 14));
        m.insert(862, NaiveDate::from_ymd(862, 2, 7));
        m.insert(863, NaiveDate::from_ymd(863, 1, 27));
        m.insert(864, NaiveDate::from_ymd(864, 2, 15));
        m.insert(865, NaiveDate::from_ymd(865, 2, 4));
        m.insert(866, NaiveDate::from_ymd(866, 1, 25));
        m.insert(867, NaiveDate::from_ymd(867, 2, 13));
        m.insert(868, NaiveDate::from_ymd(868, 2, 2));
        m.insert(869, NaiveDate::from_ymd(869, 2, 19));
        m.insert(870, NaiveDate::from_ymd(870, 2, 9));
        m.insert(871, NaiveDate::from_ymd(871, 1, 29));
        m.insert(872, NaiveDate::from_ymd(872, 2, 17));
        m.insert(873, NaiveDate::from_ymd(873, 2, 6));
        m.insert(874, NaiveDate::from_ymd(874, 1, 27));
        m.insert(875, NaiveDate::from_ymd(875, 2, 14));
        m.insert(876, NaiveDate::from_ymd(876, 2, 3));
        m.insert(877, NaiveDate::from_ymd(877, 1, 22));
        m.insert(878, NaiveDate::from_ymd(878, 2, 10));
        m.insert(879, NaiveDate::from_ymd(879, 1, 30));
        m.insert(880, NaiveDate::from_ymd(880, 2, 18));
        m.insert(881, NaiveDate::from_ymd(881, 2, 7));
        m.insert(882, NaiveDate::from_ymd(882, 1, 27));
        m.insert(883, NaiveDate::from_ymd(883, 2, 15));
        m.insert(884, NaiveDate::from_ymd(884, 2, 5));
        m.insert(885, NaiveDate::from_ymd(885, 1, 24));
        m.insert(886, NaiveDate::from_ymd(886, 2, 12));
        m.insert(887, NaiveDate::from_ymd(887, 2, 1));
        m.insert(888, NaiveDate::from_ymd(888, 2, 20));
        m.insert(889, NaiveDate::from_ymd(889, 2, 8));
        m.insert(890, NaiveDate::from_ymd(890, 1, 29));
        m.insert(891, NaiveDate::from_ymd(891, 2, 16));
        m.insert(892, NaiveDate::from_ymd(892, 2, 7));
        m.insert(893, NaiveDate::from_ymd(893, 1, 26));
        m.insert(894, NaiveDate::from_ymd(894, 2, 14));
        m.insert(895, NaiveDate::from_ymd(895, 2, 3));
        m.insert(896, NaiveDate::from_ymd(896, 1, 23));
        m.insert(897, NaiveDate::from_ymd(897, 2, 10));
        m.insert(898, NaiveDate::from_ymd(898, 1, 30));
        m.insert(899, NaiveDate::from_ymd(899, 2, 18));
        m.insert(900, NaiveDate::from_ymd(900, 2, 8));
        m.insert(901, NaiveDate::from_ymd(901, 1, 28));
        m.insert(902, NaiveDate::from_ymd(902, 2, 16));
        m.insert(903, NaiveDate::from_ymd(903, 2, 6));
        m.insert(904, NaiveDate::from_ymd(904, 1, 26));
        m.insert(905, NaiveDate::from_ymd(905, 2, 12));
        m.insert(906, NaiveDate::from_ymd(906, 2, 2));
        m.insert(907, NaiveDate::from_ymd(907, 2, 20));
        m.insert(908, NaiveDate::from_ymd(908, 2, 10));
        m.insert(909, NaiveDate::from_ymd(909, 1, 30));
        m.insert(910, NaiveDate::from_ymd(910, 2, 18));
        m.insert(911, NaiveDate::from_ymd(911, 2, 7));
        m.insert(912, NaiveDate::from_ymd(912, 1, 27));
        m.insert(913, NaiveDate::from_ymd(913, 2, 14));
        m.insert(914, NaiveDate::from_ymd(914, 2, 3));
        m.insert(915, NaiveDate::from_ymd(915, 1, 23));
        m.insert(916, NaiveDate::from_ymd(916, 2, 10));
        m.insert(917, NaiveDate::from_ymd(917, 1, 31));
        m.insert(918, NaiveDate::from_ymd(918, 2, 19));
        m.insert(919, NaiveDate::from_ymd(919, 2, 9));
        m.insert(920, NaiveDate::from_ymd(920, 1, 29));
        m.insert(921, NaiveDate::from_ymd(921, 2, 16));
        m.insert(922, NaiveDate::from_ymd(922, 2, 5));
        m.insert(923, NaiveDate::from_ymd(923, 1, 25));
        m.insert(924, NaiveDate::from_ymd(924, 2, 13));
        m.insert(925, NaiveDate::from_ymd(925, 2, 1));
        m.insert(926, NaiveDate::from_ymd(926, 2, 20));
        m.insert(927, NaiveDate::from_ymd(927, 2, 10));
        m.insert(928, NaiveDate::from_ymd(928, 1, 31));
        m.insert(929, NaiveDate::from_ymd(929, 2, 18));
        m.insert(930, NaiveDate::from_ymd(930, 2, 7));
        m.insert(931, NaiveDate::from_ymd(931, 1, 27));
        m.insert(932, NaiveDate::from_ymd(932, 2, 14));
        m.insert(933, NaiveDate::from_ymd(933, 2, 3));
        m.insert(934, NaiveDate::from_ymd(934, 1, 23));
        m.insert(935, NaiveDate::from_ymd(935, 2, 11));
        m.insert(936, NaiveDate::from_ymd(936, 2, 1));
        m.insert(937, NaiveDate::from_ymd(937, 2, 18));
        m.insert(938, NaiveDate::from_ymd(938, 2, 7));
        m.insert(939, NaiveDate::from_ymd(939, 1, 28));
        m.insert(940, NaiveDate::from_ymd(940, 2, 16));
        m.insert(941, NaiveDate::from_ymd(941, 2, 4));
        m.insert(942, NaiveDate::from_ymd(942, 1, 25));
        m.insert(943, NaiveDate::from_ymd(943, 2, 13));
        m.insert(944, NaiveDate::from_ymd(944, 2, 2));
        m.insert(945, NaiveDate::from_ymd(945, 2, 20));
        m.insert(946, NaiveDate::from_ymd(946, 2, 10));
        m.insert(947, NaiveDate::from_ymd(947, 1, 30));
        m.insert(948, NaiveDate::from_ymd(948, 2, 18));
        m.insert(949, NaiveDate::from_ymd(949, 2, 6));
        m.insert(950, NaiveDate::from_ymd(950, 1, 26));
        m.insert(951, NaiveDate::from_ymd(951, 2, 14));
        m.insert(952, NaiveDate::from_ymd(952, 2, 4));
        m.insert(953, NaiveDate::from_ymd(953, 1, 23));
        m.insert(954, NaiveDate::from_ymd(954, 2, 11));
        m.insert(955, NaiveDate::from_ymd(955, 2, 1));
        m.insert(956, NaiveDate::from_ymd(956, 2, 20));
        m.insert(957, NaiveDate::from_ymd(957, 2, 8));
        m.insert(958, NaiveDate::from_ymd(958, 1, 28));
        m.insert(959, NaiveDate::from_ymd(959, 2, 16));
        m.insert(960, NaiveDate::from_ymd(960, 2, 5));
        m.insert(961, NaiveDate::from_ymd(961, 1, 25));
        m.insert(962, NaiveDate::from_ymd(962, 2, 13));
        m.insert(963, NaiveDate::from_ymd(963, 2, 3));
        m.insert(964, NaiveDate::from_ymd(964, 2, 21));
        m.insert(965, NaiveDate::from_ymd(965, 2, 9));
        m.insert(966, NaiveDate::from_ymd(966, 1, 30));
        m.insert(967, NaiveDate::from_ymd(967, 2, 17));
        m.insert(968, NaiveDate::from_ymd(968, 2, 7));
        m.insert(969, NaiveDate::from_ymd(969, 1, 26));
        m.insert(970, NaiveDate::from_ymd(970, 2, 14));
        m.insert(971, NaiveDate::from_ymd(971, 2, 4));
        m.insert(972, NaiveDate::from_ymd(972, 1, 24));
        m.insert(973, NaiveDate::from_ymd(973, 2, 11));
        m.insert(974, NaiveDate::from_ymd(974, 1, 31));
        m.insert(975, NaiveDate::from_ymd(975, 2, 19));
        m.insert(976, NaiveDate::from_ymd(976, 2, 8));
        m.insert(977, NaiveDate::from_ymd(977, 1, 27));
        m.insert(978, NaiveDate::from_ymd(978, 2, 15));
        m.insert(979, NaiveDate::from_ymd(979, 2, 5));
        m.insert(980, NaiveDate::from_ymd(980, 1, 26));
        m.insert(981, NaiveDate::from_ymd(981, 2, 13));
        m.insert(982, NaiveDate::from_ymd(982, 2, 2));
        m.insert(983, NaiveDate::from_ymd(983, 2, 21));
        m.insert(984, NaiveDate::from_ymd(984, 2, 10));
        m.insert(985, NaiveDate::from_ymd(985, 1, 29));
        m.insert(986, NaiveDate::from_ymd(986, 2, 17));
        m.insert(987, NaiveDate::from_ymd(987, 2, 6));
        m.insert(988, NaiveDate::from_ymd(988, 1, 27));
        m.insert(989, NaiveDate::from_ymd(989, 2, 14));
        m.insert(990, NaiveDate::from_ymd(990, 2, 4));
        m.insert(991, NaiveDate::from_ymd(991, 1, 24));
        m.insert(992, NaiveDate::from_ymd(992, 2, 12));
        m.insert(993, NaiveDate::from_ymd(993, 1, 31));
        m.insert(994, NaiveDate::from_ymd(994, 2, 18));
        m.insert(995, NaiveDate::from_ymd(995, 2, 8));
        m.insert(996, NaiveDate::from_ymd(996, 1, 28));
        m.insert(997, NaiveDate::from_ymd(997, 2, 15));
        m.insert(998, NaiveDate::from_ymd(998, 2, 5));
        m.insert(999, NaiveDate::from_ymd(999, 1, 25));
        m.insert(1000, NaiveDate::from_ymd(1000, 2, 13));
        m.insert(1001, NaiveDate::from_ymd(1001, 2, 2));
        m.insert(1002, NaiveDate::from_ymd(1002, 2, 21));
        m.insert(1003, NaiveDate::from_ymd(1003, 2, 10));
        m.insert(1004, NaiveDate::from_ymd(1004, 1, 31));
        m.insert(1005, NaiveDate::from_ymd(1005, 2, 18));
        m.insert(1006, NaiveDate::from_ymd(1006, 2, 7));
        m.insert(1007, NaiveDate::from_ymd(1007, 1, 28));
        m.insert(1008, NaiveDate::from_ymd(1008, 2, 16));
        m.insert(1009, NaiveDate::from_ymd(1009, 2, 4));
        m.insert(1010, NaiveDate::from_ymd(1010, 1, 24));
        m.insert(1011, NaiveDate::from_ymd(1011, 2, 12));
        m.insert(1012, NaiveDate::from_ymd(1012, 2, 1));
        m.insert(1013, NaiveDate::from_ymd(1013, 2, 19));
        m.insert(1014, NaiveDate::from_ymd(1014, 2, 9));
        m.insert(1015, NaiveDate::from_ymd(1015, 1, 29));
        m.insert(1016, NaiveDate::from_ymd(1016, 2, 17));
        m.insert(1017, NaiveDate::from_ymd(1017, 2, 6));
        m.insert(1018, NaiveDate::from_ymd(1018, 1, 26));
        m.insert(1019, NaiveDate::from_ymd(1019, 2, 16));
        m.insert(1020, NaiveDate::from_ymd(1020, 2, 3));
        m.insert(1021, NaiveDate::from_ymd(1021, 2, 21));
        m.insert(1022, NaiveDate::from_ymd(1022, 2, 10));
        m.insert(1023, NaiveDate::from_ymd(1023, 1, 31));
        m.insert(1024, NaiveDate::from_ymd(1024, 2, 19));
        m.insert(1025, NaiveDate::from_ymd(1025, 2, 7));
        m.insert(1026, NaiveDate::from_ymd(1026, 1, 28));
        m.insert(1027, NaiveDate::from_ymd(1027, 2, 16));
        m.insert(1028, NaiveDate::from_ymd(1028, 2, 5));
        m.insert(1029, NaiveDate::from_ymd(1029, 1, 24));
        m.insert(1030, NaiveDate::from_ymd(1030, 2, 12));
        m.insert(1031, NaiveDate::from_ymd(1031, 2, 1));
        m.insert(1032, NaiveDate::from_ymd(1032, 2, 20));
        m.insert(1033, NaiveDate::from_ymd(1033, 2, 9));
        m.insert(1034, NaiveDate::from_ymd(1034, 1, 29));
        m.insert(1035, NaiveDate::from_ymd(1035, 2, 17));
        m.insert(1036, NaiveDate::from_ymd(1036, 2, 6));
        m.insert(1037, NaiveDate::from_ymd(1037, 1, 25));
        m.insert(1038, NaiveDate::from_ymd(1038, 2, 13));
        m.insert(1039, NaiveDate::from_ymd(1039, 2, 3));
        m.insert(1040, NaiveDate::from_ymd(1040, 2, 21));
        m.insert(1041, NaiveDate::from_ymd(1041, 2, 10));
        m.insert(1042, NaiveDate::from_ymd(1042, 1, 31));
        m.insert(1043, NaiveDate::from_ymd(1043, 2, 19));
        m.insert(1044, NaiveDate::from_ymd(1044, 2, 8));
        m.insert(1045, NaiveDate::from_ymd(1045, 1, 27));
        m.insert(1046, NaiveDate::from_ymd(1046, 2, 15));
        m.insert(1047, NaiveDate::from_ymd(1047, 2, 4));
        m.insert(1048, NaiveDate::from_ymd(1048, 1, 24));
        m.insert(1049, NaiveDate::from_ymd(1049, 2, 11));
        m.insert(1050, NaiveDate::from_ymd(1050, 2, 1));
        m.insert(1051, NaiveDate::from_ymd(1051, 2, 20));
        m.insert(1052, NaiveDate::from_ymd(1052, 2, 10));
        m.insert(1053, NaiveDate::from_ymd(1053, 1, 29));
        m.insert(1054, NaiveDate::from_ymd(1054, 2, 17));
        m.insert(1055, NaiveDate::from_ymd(1055, 2, 6));
        m.insert(1056, NaiveDate::from_ymd(1056, 1, 26));
        m.insert(1057, NaiveDate::from_ymd(1057, 2, 13));
        m.insert(1058, NaiveDate::from_ymd(1058, 2, 2));
        m.insert(1059, NaiveDate::from_ymd(1059, 2, 21));
        m.insert(1060, NaiveDate::from_ymd(1060, 2, 11));
        m.insert(1061, NaiveDate::from_ymd(1061, 1, 30));
        m.insert(1062, NaiveDate::from_ymd(1062, 2, 18));
        m.insert(1063, NaiveDate::from_ymd(1063, 2, 7));
        m.insert(1064, NaiveDate::from_ymd(1064, 1, 27));
        m.insert(1065, NaiveDate::from_ymd(1065, 2, 14));
        m.insert(1066, NaiveDate::from_ymd(1066, 2, 4));
        m.insert(1067, NaiveDate::from_ymd(1067, 1, 24));
        m.insert(1068, NaiveDate::from_ymd(1068, 2, 12));
        m.insert(1069, NaiveDate::from_ymd(1069, 2, 1));
        m.insert(1070, NaiveDate::from_ymd(1070, 2, 20));
        m.insert(1071, NaiveDate::from_ymd(1071, 2, 9));
        m.insert(1072, NaiveDate::from_ymd(1072, 1, 29));
        m.insert(1073, NaiveDate::from_ymd(1073, 2, 16));
        m.insert(1074, NaiveDate::from_ymd(1074, 2, 5));
        m.insert(1075, NaiveDate::from_ymd(1075, 1, 26));
        m.insert(1076, NaiveDate::from_ymd(1076, 2, 14));
        m.insert(1077, NaiveDate::from_ymd(1077, 2, 2));
        m.insert(1078, NaiveDate::from_ymd(1078, 2, 21));
        m.insert(1079, NaiveDate::from_ymd(1079, 2, 11));
        m.insert(1080, NaiveDate::from_ymd(1080, 1, 31));
        m.insert(1081, NaiveDate::from_ymd(1081, 2, 18));
        m.insert(1082, NaiveDate::from_ymd(1082, 2, 7));
        m.insert(1083, NaiveDate::from_ymd(1083, 1, 27));
        m.insert(1084, NaiveDate::from_ymd(1084, 2, 15));
        m.insert(1085, NaiveDate::from_ymd(1085, 2, 4));
        m.insert(1086, NaiveDate::from_ymd(1086, 1, 24));
        m.insert(1087, NaiveDate::from_ymd(1087, 2, 12));
        m.insert(1088, NaiveDate::from_ymd(1088, 2, 2));
        m.insert(1089, NaiveDate::from_ymd(1089, 2, 20));
        m.insert(1090, NaiveDate::from_ymd(1090, 2, 9));
        m.insert(1091, NaiveDate::from_ymd(1091, 1, 29));
        m.insert(1092, NaiveDate::from_ymd(1092, 2, 16));
        m.insert(1093, NaiveDate::from_ymd(1093, 2, 5));
        m.insert(1094, NaiveDate::from_ymd(1094, 1, 25));
        m.insert(1095, NaiveDate::from_ymd(1095, 2, 14));
        m.insert(1096, NaiveDate::from_ymd(1096, 2, 3));
        m.insert(1097, NaiveDate::from_ymd(1097, 1, 22));
        m.insert(1098, NaiveDate::from_ymd(1098, 2, 10));
        m.insert(1099, NaiveDate::from_ymd(1099, 1, 30));
        m.insert(1100, NaiveDate::from_ymd(1100, 2, 18));
        m.insert(1101, NaiveDate::from_ymd(1101, 2, 7));
        m.insert(1102, NaiveDate::from_ymd(1102, 1, 28));
        m.insert(1103, NaiveDate::from_ymd(1103, 2, 16));
        m.insert(1104, NaiveDate::from_ymd(1104, 2, 6));
        m.insert(1105, NaiveDate::from_ymd(1105, 1, 25));
        m.insert(1106, NaiveDate::from_ymd(1106, 2, 13));
        m.insert(1107, NaiveDate::from_ymd(1107, 2, 2));
        m.insert(1108, NaiveDate::from_ymd(1108, 2, 21));
        m.insert(1109, NaiveDate::from_ymd(1109, 2, 9));
        m.insert(1110, NaiveDate::from_ymd(1110, 1, 29));
        m.insert(1111, NaiveDate::from_ymd(1111, 2, 17));
        m.insert(1112, NaiveDate::from_ymd(1112, 2, 7));
        m.insert(1113, NaiveDate::from_ymd(1113, 1, 27));
        m.insert(1114, NaiveDate::from_ymd(1114, 2, 15));
        m.insert(1115, NaiveDate::from_ymd(1115, 2, 4));
        m.insert(1116, NaiveDate::from_ymd(1116, 1, 24));
        m.insert(1117, NaiveDate::from_ymd(1117, 2, 11));
        m.insert(1118, NaiveDate::from_ymd(1118, 1, 31));
        m.insert(1119, NaiveDate::from_ymd(1119, 2, 19));
        m.insert(1120, NaiveDate::from_ymd(1120, 2, 8));
        m.insert(1121, NaiveDate::from_ymd(1121, 1, 28));
        m.insert(1122, NaiveDate::from_ymd(1122, 2, 16));
        m.insert(1123, NaiveDate::from_ymd(1123, 2, 5));
        m.insert(1124, NaiveDate::from_ymd(1124, 1, 26));
        m.insert(1125, NaiveDate::from_ymd(1125, 2, 12));
        m.insert(1126, NaiveDate::from_ymd(1126, 2, 1));
        m.insert(1127, NaiveDate::from_ymd(1127, 2, 20));
        m.insert(1128, NaiveDate::from_ymd(1128, 2, 10));
        m.insert(1129, NaiveDate::from_ymd(1129, 1, 29));
        m.insert(1130, NaiveDate::from_ymd(1130, 2, 17));
        m.insert(1131, NaiveDate::from_ymd(1131, 2, 7));
        m.insert(1132, NaiveDate::from_ymd(1132, 1, 27));
        m.insert(1133, NaiveDate::from_ymd(1133, 2, 14));
        m.insert(1134, NaiveDate::from_ymd(1134, 2, 3));
        m.insert(1135, NaiveDate::from_ymd(1135, 2, 22));
        m.insert(1136, NaiveDate::from_ymd(1136, 2, 11));
        m.insert(1137, NaiveDate::from_ymd(1137, 1, 30));
        m.insert(1138, NaiveDate::from_ymd(1138, 2, 18));
        m.insert(1139, NaiveDate::from_ymd(1139, 2, 8));
        m.insert(1140, NaiveDate::from_ymd(1140, 1, 29));
        m.insert(1141, NaiveDate::from_ymd(1141, 2, 16));
        m.insert(1142, NaiveDate::from_ymd(1142, 2, 5));
        m.insert(1143, NaiveDate::from_ymd(1143, 1, 25));
        m.insert(1144, NaiveDate::from_ymd(1144, 2, 13));
        m.insert(1145, NaiveDate::from_ymd(1145, 2, 1));
        m.insert(1146, NaiveDate::from_ymd(1146, 2, 20));
        m.insert(1147, NaiveDate::from_ymd(1147, 2, 9));
        m.insert(1148, NaiveDate::from_ymd(1148, 1, 30));
        m.insert(1149, NaiveDate::from_ymd(1149, 2, 17));
        m.insert(1150, NaiveDate::from_ymd(1150, 2, 7));
        m.insert(1151, NaiveDate::from_ymd(1151, 1, 27));
        m.insert(1152, NaiveDate::from_ymd(1152, 2, 15));
        m.insert(1153, NaiveDate::from_ymd(1153, 2, 3));
        m.insert(1154, NaiveDate::from_ymd(1154, 2, 21));
        m.insert(1155, NaiveDate::from_ymd(1155, 2, 11));
        m.insert(1156, NaiveDate::from_ymd(1156, 1, 31));
        m.insert(1157, NaiveDate::from_ymd(1157, 2, 18));
        m.insert(1158, NaiveDate::from_ymd(1158, 2, 8));
        m.insert(1159, NaiveDate::from_ymd(1159, 1, 28));
        m.insert(1160, NaiveDate::from_ymd(1160, 2, 16));
        m.insert(1161, NaiveDate::from_ymd(1161, 2, 4));
        m.insert(1162, NaiveDate::from_ymd(1162, 1, 24));
        m.insert(1163, NaiveDate::from_ymd(1163, 2, 12));
        m.insert(1164, NaiveDate::from_ymd(1164, 2, 3));
        m.insert(1165, NaiveDate::from_ymd(1165, 2, 20));
        m.insert(1166, NaiveDate::from_ymd(1166, 2, 10));
        m.insert(1167, NaiveDate::from_ymd(1167, 1, 30));
        m.insert(1168, NaiveDate::from_ymd(1168, 2, 18));
        m.insert(1169, NaiveDate::from_ymd(1169, 2, 6));
        m.insert(1170, NaiveDate::from_ymd(1170, 1, 26));
        m.insert(1171, NaiveDate::from_ymd(1171, 2, 14));
        m.insert(1172, NaiveDate::from_ymd(1172, 2, 3));
        m.insert(1173, NaiveDate::from_ymd(1173, 2, 21));
        m.insert(1174, NaiveDate::from_ymd(1174, 2, 11));
        m.insert(1175, NaiveDate::from_ymd(1175, 1, 31));
        m.insert(1176, NaiveDate::from_ymd(1176, 2, 19));
        m.insert(1177, NaiveDate::from_ymd(1177, 2, 8));
        m.insert(1178, NaiveDate::from_ymd(1178, 1, 28));
        m.insert(1179, NaiveDate::from_ymd(1179, 2, 16));
        m.insert(1180, NaiveDate::from_ymd(1180, 2, 5));
        m.insert(1181, NaiveDate::from_ymd(1181, 1, 24));
        m.insert(1182, NaiveDate::from_ymd(1182, 2, 12));
        m.insert(1183, NaiveDate::from_ymd(1183, 2, 2));
        m.insert(1184, NaiveDate::from_ymd(1184, 2, 21));
        m.insert(1185, NaiveDate::from_ymd(1185, 2, 9));
        m.insert(1186, NaiveDate::from_ymd(1186, 1, 30));
        m.insert(1187, NaiveDate::from_ymd(1187, 2, 17));
        m.insert(1188, NaiveDate::from_ymd(1188, 2, 6));
        m.insert(1189, NaiveDate::from_ymd(1189, 1, 26));
        m.insert(1190, NaiveDate::from_ymd(1190, 2, 14));
        m.insert(1191, NaiveDate::from_ymd(1191, 2, 3));
        m.insert(1192, NaiveDate::from_ymd(1192, 2, 22));
        m.insert(1193, NaiveDate::from_ymd(1193, 2, 11));
        m.insert(1194, NaiveDate::from_ymd(1194, 1, 31));
        m.insert(1195, NaiveDate::from_ymd(1195, 2, 19));
        m.insert(1196, NaiveDate::from_ymd(1196, 2, 8));
        m.insert(1197, NaiveDate::from_ymd(1197, 1, 27));
        m.insert(1198, NaiveDate::from_ymd(1198, 2, 15));
        m.insert(1199, NaiveDate::from_ymd(1999, 2, 4));
        m.insert(1200, NaiveDate::from_ymd(1200, 1, 25));
        m.insert(1201, NaiveDate::from_ymd(1201, 2, 12));
        m.insert(1202, NaiveDate::from_ymd(1202, 2, 2));
        m.insert(1203, NaiveDate::from_ymd(1203, 2, 21));
        m.insert(1204, NaiveDate::from_ymd(1204, 2, 10));
        m.insert(1205, NaiveDate::from_ymd(1205, 1, 29));
        m.insert(1206, NaiveDate::from_ymd(1206, 2, 17));
        m.insert(1207, NaiveDate::from_ymd(1207, 2, 6));
        m.insert(1208, NaiveDate::from_ymd(1208, 1, 26));
        m.insert(1209, NaiveDate::from_ymd(1209, 2, 14));
        m.insert(1210, NaiveDate::from_ymd(1210, 2, 3));
        m.insert(1211, NaiveDate::from_ymd(1211, 1, 24));
        m.insert(1212, NaiveDate::from_ymd(1212, 2, 12));
        m.insert(1213, NaiveDate::from_ymd(1213, 1, 31));
        m.insert(1214, NaiveDate::from_ymd(1214, 2, 19));
        m.insert(1215, NaiveDate::from_ymd(1215, 2, 8));
        m.insert(1216, NaiveDate::from_ymd(1216, 1, 28));
        m.insert(1217, NaiveDate::from_ymd(1217, 2, 15));
        m.insert(1218, NaiveDate::from_ymd(1218, 2, 4));
        m.insert(1219, NaiveDate::from_ymd(1219, 1, 25));
        m.insert(1220, NaiveDate::from_ymd(1220, 2, 13));
        m.insert(1221, NaiveDate::from_ymd(1221, 2, 1));
        m.insert(1222, NaiveDate::from_ymd(1222, 2, 20));
        m.insert(1223, NaiveDate::from_ymd(1223, 2, 9));
        m.insert(1224, NaiveDate::from_ymd(1224, 1, 29));
        m.insert(1225, NaiveDate::from_ymd(1225, 2, 16));
        m.insert(1226, NaiveDate::from_ymd(1226, 2, 6));
        m.insert(1227, NaiveDate::from_ymd(1227, 1, 26));
        m.insert(1228, NaiveDate::from_ymd(1228, 2, 16));
        m.insert(1229, NaiveDate::from_ymd(1229, 2, 3));
        m.insert(1230, NaiveDate::from_ymd(1230, 1, 23));
        m.insert(1231, NaiveDate::from_ymd(1231, 2, 11));
        m.insert(1232, NaiveDate::from_ymd(1232, 1, 31));
        m.insert(1233, NaiveDate::from_ymd(1233, 2, 18));
        m.insert(1234, NaiveDate::from_ymd(1234, 2, 7));
        m.insert(1235, NaiveDate::from_ymd(1235, 1, 28));
        m.insert(1236, NaiveDate::from_ymd(1236, 2, 16));
        m.insert(1237, NaiveDate::from_ymd(1237, 2, 4));
        m.insert(1238, NaiveDate::from_ymd(1238, 1, 25));
        m.insert(1239, NaiveDate::from_ymd(1239, 2, 13));
        m.insert(1240, NaiveDate::from_ymd(1240, 2, 2));
        m.insert(1241, NaiveDate::from_ymd(1241, 2, 20));
        m.insert(1242, NaiveDate::from_ymd(1242, 2, 9));
        m.insert(1243, NaiveDate::from_ymd(1243, 1, 29));
        m.insert(1244, NaiveDate::from_ymd(1244, 2, 17));
        m.insert(1245, NaiveDate::from_ymd(1245, 2, 6));
        m.insert(1246, NaiveDate::from_ymd(1246, 1, 26));
        m.insert(1247, NaiveDate::from_ymd(1247, 2, 14));
        m.insert(1248, NaiveDate::from_ymd(1248, 2, 4));
        m.insert(1249, NaiveDate::from_ymd(1249, 2, 21));
        m.insert(1250, NaiveDate::from_ymd(1250, 2, 10));
        m.insert(1251, NaiveDate::from_ymd(1251, 1, 31));
        m.insert(1252, NaiveDate::from_ymd(1252, 2, 19));
        m.insert(1253, NaiveDate::from_ymd(1253, 2, 7));
        m.insert(1254, NaiveDate::from_ymd(1254, 1, 29));
        m.insert(1255, NaiveDate::from_ymd(1255, 2, 16));
        m.insert(1256, NaiveDate::from_ymd(1256, 2, 5));
        m.insert(1257, NaiveDate::from_ymd(1257, 1, 24));
        m.insert(1258, NaiveDate::from_ymd(1258, 2, 12));
        m.insert(1259, NaiveDate::from_ymd(1259, 2, 1));
        m.insert(1260, NaiveDate::from_ymd(1260, 2, 20));
        m.insert(1261, NaiveDate::from_ymd(1261, 2, 8));
        m.insert(1262, NaiveDate::from_ymd(1262, 1, 29));
        m.insert(1263, NaiveDate::from_ymd(1263, 2, 17));
        m.insert(1264, NaiveDate::from_ymd(1264, 2, 7));
        m.insert(1265, NaiveDate::from_ymd(1265, 1, 26));
        m.insert(1266, NaiveDate::from_ymd(1266, 2, 14));
        m.insert(1267, NaiveDate::from_ymd(1267, 2, 3));
        m.insert(1268, NaiveDate::from_ymd(1268, 1, 23));
        m.insert(1269, NaiveDate::from_ymd(1269, 2, 10));
        m.insert(1270, NaiveDate::from_ymd(1270, 1, 30));
        m.insert(1271, NaiveDate::from_ymd(1271, 2, 18));
        m.insert(1272, NaiveDate::from_ymd(1272, 2, 8));
        m.insert(1273, NaiveDate::from_ymd(1273, 1, 28));
        m.insert(1274, NaiveDate::from_ymd(1274, 2, 16));
        m.insert(1275, NaiveDate::from_ymd(1275, 2, 5));
        m.insert(1276, NaiveDate::from_ymd(1276, 1, 25));
        m.insert(1277, NaiveDate::from_ymd(1277, 2, 12));
        m.insert(1278, NaiveDate::from_ymd(1278, 2, 1));
        m.insert(1279, NaiveDate::from_ymd(1279, 2, 20));
        m.insert(1280, NaiveDate::from_ymd(1280, 2, 9));
        m.insert(1281, NaiveDate::from_ymd(1281, 1, 29));
        m.insert(1282, NaiveDate::from_ymd(1282, 2, 17));
        m.insert(1283, NaiveDate::from_ymd(1283, 2, 6));
        m.insert(1284, NaiveDate::from_ymd(1284, 1, 27));
        m.insert(1285, NaiveDate::from_ymd(1285, 2, 13));
        m.insert(1286, NaiveDate::from_ymd(1286, 2, 2));
        m.insert(1287, NaiveDate::from_ymd(1287, 2, 21));
        m.insert(1288, NaiveDate::from_ymd(1288, 2, 11));
        m.insert(1289, NaiveDate::from_ymd(1289, 1, 30));
        m.insert(1290, NaiveDate::from_ymd(1290, 2, 18));
        m.insert(1291, NaiveDate::from_ymd(1291, 2, 8));
        m.insert(1292, NaiveDate::from_ymd(1292, 1, 28));
        m.insert(1293, NaiveDate::from_ymd(1293, 2, 15));
        m.insert(1294, NaiveDate::from_ymd(1294, 2, 4));
        m.insert(1295, NaiveDate::from_ymd(1295, 1, 24));
        m.insert(1296, NaiveDate::from_ymd(1296, 2, 12));
        m.insert(1297, NaiveDate::from_ymd(1297, 2, 1));
        m.insert(1298, NaiveDate::from_ymd(1298, 2, 20));
        m.insert(1299, NaiveDate::from_ymd(1299, 2, 9));
        m.insert(1300, NaiveDate::from_ymd(1300, 1, 30));
        m.insert(1301, NaiveDate::from_ymd(1301, 2, 18));
        m.insert(1302, NaiveDate::from_ymd(1302, 2, 7));
        m.insert(1303, NaiveDate::from_ymd(1303, 1, 27));
        m.insert(1304, NaiveDate::from_ymd(1304, 2, 15));
        m.insert(1305, NaiveDate::from_ymd(1305, 2, 3));
        m.insert(1306, NaiveDate::from_ymd(1306, 2, 22));
        m.insert(1307, NaiveDate::from_ymd(1307, 2, 12));
        m.insert(1308, NaiveDate::from_ymd(1308, 2, 1));
        m.insert(1309, NaiveDate::from_ymd(1309, 2, 19));
        m.insert(1310, NaiveDate::from_ymd(1310, 2, 9));
        m.insert(1311, NaiveDate::from_ymd(1311, 1, 29));
        m.insert(1312, NaiveDate::from_ymd(1312, 2, 16));
        m.insert(1313, NaiveDate::from_ymd(1313, 2, 5));
        m.insert(1314, NaiveDate::from_ymd(1314, 1, 25));
        m.insert(1315, NaiveDate::from_ymd(1315, 2, 13));
        m.insert(1316, NaiveDate::from_ymd(1316, 2, 3));
        m.insert(1317, NaiveDate::from_ymd(1317, 2, 20));
        m.insert(1318, NaiveDate::from_ymd(1318, 2, 10));
        m.insert(1319, NaiveDate::from_ymd(1319, 1, 30));
        m.insert(1320, NaiveDate::from_ymd(1320, 2, 18));
        m.insert(1321, NaiveDate::from_ymd(1321, 2, 6));
        m.insert(1322, NaiveDate::from_ymd(1322, 1, 26));
        m.insert(1323, NaiveDate::from_ymd(1323, 2, 14));
        m.insert(1324, NaiveDate::from_ymd(1324, 2, 4));
        m.insert(1325, NaiveDate::from_ymd(1325, 1, 24));
        m.insert(1326, NaiveDate::from_ymd(1326, 2, 12));
        m.insert(1327, NaiveDate::from_ymd(1327, 2, 1));
        m.insert(1328, NaiveDate::from_ymd(1328, 2, 20));
        m.insert(1329, NaiveDate::from_ymd(1329, 2, 8));
        m.insert(1330, NaiveDate::from_ymd(1330, 1, 28));
        m.insert(1331, NaiveDate::from_ymd(1331, 2, 16));
        m.insert(1332, NaiveDate::from_ymd(1332, 2, 5));
        m.insert(1333, NaiveDate::from_ymd(1333, 1, 15));
        m.insert(1334, NaiveDate::from_ymd(1334, 2, 13));
        m.insert(1335, NaiveDate::from_ymd(1335, 2, 3));
        m.insert(1336, NaiveDate::from_ymd(1336, 2, 21));
        m.insert(1337, NaiveDate::from_ymd(1337, 2, 10));
        m.insert(1338, NaiveDate::from_ymd(1338, 1, 30));
        m.insert(1339, NaiveDate::from_ymd(1339, 2, 18));
        m.insert(1340, NaiveDate::from_ymd(1340, 2, 7));
        m.insert(1341, NaiveDate::from_ymd(1341, 1, 26));
        m.insert(1342, NaiveDate::from_ymd(1342, 2, 14));
        m.insert(1343, NaiveDate::from_ymd(1343, 2, 4));
        m.insert(1344, NaiveDate::from_ymd(1344, 1, 24));
        m.insert(1345, NaiveDate::from_ymd(1345, 2, 11));
        m.insert(1346, NaiveDate::from_ymd(1346, 2, 1));
        m.insert(1347, NaiveDate::from_ymd(1347, 2, 19));
        m.insert(1348, NaiveDate::from_ymd(1348, 2, 8));
        m.insert(1349, NaiveDate::from_ymd(1349, 1, 28));
        m.insert(1350, NaiveDate::from_ymd(1350, 2, 16));
        m.insert(1351, NaiveDate::from_ymd(1351, 2, 5));
        m.insert(1352, NaiveDate::from_ymd(1352, 1, 26));
        m.insert(1353, NaiveDate::from_ymd(1353, 2, 13));
        m.insert(1354, NaiveDate::from_ymd(1354, 2, 2));
        m.insert(1355, NaiveDate::from_ymd(1355, 2, 21));
        m.insert(1356, NaiveDate::from_ymd(1356, 2, 10));
        m.insert(1357, NaiveDate::from_ymd(1357, 1, 29));
        m.insert(1358, NaiveDate::from_ymd(1358, 2, 17));
        m.insert(1359, NaiveDate::from_ymd(1359, 2, 7));
        m.insert(1360, NaiveDate::from_ymd(1360, 1, 27));
        m.insert(1361, NaiveDate::from_ymd(1361, 2, 14));
        m.insert(1362, NaiveDate::from_ymd(1362, 2, 4));
        m.insert(1363, NaiveDate::from_ymd(1363, 1, 24));
        m.insert(1364, NaiveDate::from_ymd(1364, 2, 12));
        m.insert(1365, NaiveDate::from_ymd(1365, 1, 31));
        m.insert(1366, NaiveDate::from_ymd(1366, 2, 19));
        m.insert(1367, NaiveDate::from_ymd(1367, 2, 8));
        m.insert(1368, NaiveDate::from_ymd(1368, 1, 29));
        m.insert(1369, NaiveDate::from_ymd(1369, 2, 16));
        m.insert(1370, NaiveDate::from_ymd(1370, 2, 5));
        m.insert(1371, NaiveDate::from_ymd(1371, 1, 26));
        m.insert(1372, NaiveDate::from_ymd(1372, 2, 14));
        m.insert(1373, NaiveDate::from_ymd(1373, 2, 2));
        m.insert(1374, NaiveDate::from_ymd(1374, 2, 20));
        m.insert(1375, NaiveDate::from_ymd(1375, 2, 10));
        m.insert(1376, NaiveDate::from_ymd(1376, 1, 30));
        m.insert(1377, NaiveDate::from_ymd(1377, 2, 17));
        m.insert(1378, NaiveDate::from_ymd(1378, 2, 7));
        m.insert(1379, NaiveDate::from_ymd(1379, 1, 27));
        m.insert(1380, NaiveDate::from_ymd(1380, 2, 15));
        m.insert(1381, NaiveDate::from_ymd(1381, 2, 3));
        m.insert(1382, NaiveDate::from_ymd(1382, 1, 23));
        m.insert(1383, NaiveDate::from_ymd(1383, 2, 11));
        m.insert(1384, NaiveDate::from_ymd(1384, 1, 31));
        m.insert(1385, NaiveDate::from_ymd(1385, 2, 18));
        m.insert(1386, NaiveDate::from_ymd(1386, 2, 8));
        m.insert(1387, NaiveDate::from_ymd(1387, 1, 29));
        m.insert(1388, NaiveDate::from_ymd(1388, 2, 17));
        m.insert(1389, NaiveDate::from_ymd(1389, 2, 5));
        m.insert(1390, NaiveDate::from_ymd(1390, 1, 15));
        m.insert(1391, NaiveDate::from_ymd(1391, 2, 13));
        m.insert(1392, NaiveDate::from_ymd(1392, 2, 2));
        m.insert(1393, NaiveDate::from_ymd(1393, 2, 20));
        m.insert(1394, NaiveDate::from_ymd(1394, 2, 9));
        m.insert(1395, NaiveDate::from_ymd(1395, 1, 30));
        m.insert(1396, NaiveDate::from_ymd(1396, 2, 18));
        m.insert(1397, NaiveDate::from_ymd(1397, 2, 7));
        m.insert(1398, NaiveDate::from_ymd(1398, 1, 27));
        m.insert(1399, NaiveDate::from_ymd(1399, 2, 15));
        m.insert(1400, NaiveDate::from_ymd(1400, 2, 4));
        m.insert(1401, NaiveDate::from_ymd(1401, 1, 24));
        m.insert(1402, NaiveDate::from_ymd(1402, 2, 12));
        m.insert(1403, NaiveDate::from_ymd(1403, 2, 1));
        m.insert(1404, NaiveDate::from_ymd(1404, 2, 20));
        m.insert(1405, NaiveDate::from_ymd(1405, 2, 9));
        m.insert(1406, NaiveDate::from_ymd(1406, 1, 29));
        m.insert(1407, NaiveDate::from_ymd(1407, 2, 17));
        m.insert(1408, NaiveDate::from_ymd(1408, 2, 7));
        m.insert(1409, NaiveDate::from_ymd(1409, 1, 26));
        m.insert(1410, NaiveDate::from_ymd(1410, 2, 13));
        m.insert(1411, NaiveDate::from_ymd(1411, 2, 3));
        m.insert(1412, NaiveDate::from_ymd(1412, 2, 22));
        m.insert(1413, NaiveDate::from_ymd(1413, 2, 10));
        m.insert(1414, NaiveDate::from_ymd(1414, 1, 31));
        m.insert(1415, NaiveDate::from_ymd(1415, 2, 19));
        m.insert(1416, NaiveDate::from_ymd(1416, 2, 8));
        m.insert(1417, NaiveDate::from_ymd(1417, 1, 27));
        m.insert(1418, NaiveDate::from_ymd(1418, 2, 15));
        m.insert(1419, NaiveDate::from_ymd(1419, 2, 4));
        m.insert(1420, NaiveDate::from_ymd(1420, 1, 25));
        m.insert(1421, NaiveDate::from_ymd(1421, 2, 12));
        m.insert(1422, NaiveDate::from_ymd(1422, 2, 1));
        m.insert(1423, NaiveDate::from_ymd(1423, 2, 20));
        m.insert(1424, NaiveDate::from_ymd(1424, 2, 10));
        m.insert(1425, NaiveDate::from_ymd(1425, 1, 29));
        m.insert(1426, NaiveDate::from_ymd(1426, 2, 17));
        m.insert(1427, NaiveDate::from_ymd(1427, 2, 6));
        m.insert(1428, NaiveDate::from_ymd(1428, 1, 26));
        m.insert(1429, NaiveDate::from_ymd(1429, 2, 13));
        m.insert(1430, NaiveDate::from_ymd(1430, 2, 3));
        m.insert(1431, NaiveDate::from_ymd(1431, 2, 22));
        m.insert(1432, NaiveDate::from_ymd(1432, 2, 11));
        m.insert(1433, NaiveDate::from_ymd(1433, 1, 31));
        m.insert(1434, NaiveDate::from_ymd(1434, 2, 19));
        m.insert(1435, NaiveDate::from_ymd(1435, 2, 8));
        m.insert(1436, NaiveDate::from_ymd(1436, 1, 28));
        m.insert(1437, NaiveDate::from_ymd(1437, 2, 15));
        m.insert(1438, NaiveDate::from_ymd(1438, 2, 4));
        m.insert(1439, NaiveDate::from_ymd(1439, 1, 25));
        m.insert(1440, NaiveDate::from_ymd(1440, 2, 13));
        m.insert(1441, NaiveDate::from_ymd(1441, 2, 1));
        m.insert(1442, NaiveDate::from_ymd(1442, 2, 20));
        m.insert(1443, NaiveDate::from_ymd(1443, 2, 9));
        m.insert(1444, NaiveDate::from_ymd(1444, 1, 29));
        m.insert(1445, NaiveDate::from_ymd(1445, 2, 16));
        m.insert(1446, NaiveDate::from_ymd(1446, 2, 5));
        m.insert(1447, NaiveDate::from_ymd(1447, 1, 16));
        m.insert(1448, NaiveDate::from_ymd(1448, 2, 14));
        m.insert(1449, NaiveDate::from_ymd(1449, 2, 3));
        m.insert(1450, NaiveDate::from_ymd(1450, 2, 21));
        m.insert(1451, NaiveDate::from_ymd(1451, 2, 11));
        m.insert(1452, NaiveDate::from_ymd(1452, 1, 31));
        m.insert(1453, NaiveDate::from_ymd(1453, 2, 18));
        m.insert(1454, NaiveDate::from_ymd(1454, 2, 7));
        m.insert(1455, NaiveDate::from_ymd(1455, 1, 27));
        m.insert(1456, NaiveDate::from_ymd(1456, 2, 15));
        m.insert(1457, NaiveDate::from_ymd(1457, 2, 4));
        m.insert(1458, NaiveDate::from_ymd(1458, 1, 25));
        m.insert(1459, NaiveDate::from_ymd(1459, 2, 13));
        m.insert(1460, NaiveDate::from_ymd(1460, 2, 2));
        m.insert(1461, NaiveDate::from_ymd(1461, 2, 20));
        m.insert(1462, NaiveDate::from_ymd(1462, 2, 9));
        m.insert(1463, NaiveDate::from_ymd(1463, 1, 29));
        m.insert(1464, NaiveDate::from_ymd(1464, 2, 17));
        m.insert(1465, NaiveDate::from_ymd(1465, 2, 5));
        m.insert(1466, NaiveDate::from_ymd(1466, 1, 26));
        m.insert(1467, NaiveDate::from_ymd(1467, 2, 14));
        m.insert(1468, NaiveDate::from_ymd(1468, 2, 3));
        m.insert(1469, NaiveDate::from_ymd(1469, 2, 21));
        m.insert(1470, NaiveDate::from_ymd(1470, 2, 11));
        m.insert(1471, NaiveDate::from_ymd(1471, 1, 30));
        m.insert(1472, NaiveDate::from_ymd(1472, 2, 18));
        m.insert(1473, NaiveDate::from_ymd(1473, 2, 7));
        m.insert(1474, NaiveDate::from_ymd(1474, 1, 27));
        m.insert(1475, NaiveDate::from_ymd(1475, 2, 15));
        m.insert(1476, NaiveDate::from_ymd(1476, 2, 5));
        m.insert(1477, NaiveDate::from_ymd(1477, 1, 24));
        m.insert(1478, NaiveDate::from_ymd(1478, 2, 12));
        m.insert(1479, NaiveDate::from_ymd(1479, 2, 1));
        m.insert(1480, NaiveDate::from_ymd(1480, 2, 20));
        m.insert(1481, NaiveDate::from_ymd(1481, 2, 8));
        m.insert(1482, NaiveDate::from_ymd(1482, 1, 29));
        m.insert(1483, NaiveDate::from_ymd(1483, 2, 17));
        m.insert(1484, NaiveDate::from_ymd(1484, 2, 6));
        m.insert(1485, NaiveDate::from_ymd(1485, 1, 26));
        m.insert(1486, NaiveDate::from_ymd(1486, 2, 14));
        m.insert(1487, NaiveDate::from_ymd(1487, 2, 3));
        m.insert(1488, NaiveDate::from_ymd(1488, 2, 22));
        m.insert(1489, NaiveDate::from_ymd(1489, 2, 10));
        m.insert(1490, NaiveDate::from_ymd(1490, 1, 30));
        m.insert(1491, NaiveDate::from_ymd(1491, 2, 18));
        m.insert(1492, NaiveDate::from_ymd(1492, 2, 8));
        m.insert(1493, NaiveDate::from_ymd(1493, 1, 27));
        m.insert(1494, NaiveDate::from_ymd(1494, 2, 15));
        m.insert(1495, NaiveDate::from_ymd(1495, 2, 5));
        m.insert(1496, NaiveDate::from_ymd(1496, 1, 25));
        m.insert(1497, NaiveDate::from_ymd(1497, 2, 12));
        m.insert(1498, NaiveDate::from_ymd(1498, 2, 1));
        m.insert(1499, NaiveDate::from_ymd(1499, 2, 19));
        m.insert(1500, NaiveDate::from_ymd(1500, 2, 9));
        m.insert(1501, NaiveDate::from_ymd(1501, 1, 29));
        m.insert(1502, NaiveDate::from_ymd(1502, 2, 18));
        m.insert(1503, NaiveDate::from_ymd(1503, 2, 7));
        m.insert(1504, NaiveDate::from_ymd(1504, 1, 28));
        m.insert(1505, NaiveDate::from_ymd(1505, 2, 14));
        m.insert(1506, NaiveDate::from_ymd(1506, 2, 3));
        m.insert(1507, NaiveDate::from_ymd(1507, 2, 22));
        m.insert(1508, NaiveDate::from_ymd(1508, 2, 11));
        m.insert(1509, NaiveDate::from_ymd(1509, 1, 31));
        m.insert(1510, NaiveDate::from_ymd(1510, 2, 19));
        m.insert(1511, NaiveDate::from_ymd(1511, 2, 9));
        m.insert(1512, NaiveDate::from_ymd(1512, 1, 29));
        m.insert(1513, NaiveDate::from_ymd(1513, 2, 16));
        m.insert(1514, NaiveDate::from_ymd(1514, 2, 5));
        m.insert(1515, NaiveDate::from_ymd(1515, 1, 25));
        m.insert(1516, NaiveDate::from_ymd(1516, 2, 13));
        m.insert(1517, NaiveDate::from_ymd(1517, 2, 1));
        m.insert(1518, NaiveDate::from_ymd(1518, 2, 20));
        m.insert(1519, NaiveDate::from_ymd(1519, 2, 10));
        m.insert(1520, NaiveDate::from_ymd(1520, 1, 31));
        m.insert(1521, NaiveDate::from_ymd(1521, 2, 18));
        m.insert(1522, NaiveDate::from_ymd(1522, 2, 7));
        m.insert(1523, NaiveDate::from_ymd(1523, 1, 27));
        m.insert(1524, NaiveDate::from_ymd(1524, 2, 15));
        m.insert(1525, NaiveDate::from_ymd(1525, 2, 3));
        m.insert(1526, NaiveDate::from_ymd(1526, 2, 22));
        m.insert(1527, NaiveDate::from_ymd(1527, 2, 11));
        m.insert(1528, NaiveDate::from_ymd(1528, 2, 1));
        m.insert(1529, NaiveDate::from_ymd(1529, 2, 19));
        m.insert(1530, NaiveDate::from_ymd(1530, 2, 8));
        m.insert(1531, NaiveDate::from_ymd(1531, 1, 29));
        m.insert(1532, NaiveDate::from_ymd(1532, 2, 16));
        m.insert(1533, NaiveDate::from_ymd(1533, 2, 4));
        m.insert(1534, NaiveDate::from_ymd(1534, 1, 25));
        m.insert(1535, NaiveDate::from_ymd(1535, 2, 13));
        m.insert(1536, NaiveDate::from_ymd(1536, 2, 2));
        m.insert(1537, NaiveDate::from_ymd(1537, 2, 20));
        m.insert(1538, NaiveDate::from_ymd(1538, 2, 10));
        m.insert(1539, NaiveDate::from_ymd(1539, 1, 30));
        m.insert(1540, NaiveDate::from_ymd(1540, 2, 18));
        m.insert(1541, NaiveDate::from_ymd(1541, 2, 6));
        m.insert(1542, NaiveDate::from_ymd(1542, 1, 26));
        m.insert(1543, NaiveDate::from_ymd(1543, 2, 14));
        m.insert(1544, NaiveDate::from_ymd(1544, 2, 3));
        m.insert(1545, NaiveDate::from_ymd(1545, 2, 22));
        m.insert(1546, NaiveDate::from_ymd(1546, 2, 11));
        m.insert(1547, NaiveDate::from_ymd(1547, 2, 1));
        m.insert(1548, NaiveDate::from_ymd(1548, 2, 20));
        m.insert(1549, NaiveDate::from_ymd(1549, 2, 8));
        m.insert(1550, NaiveDate::from_ymd(1550, 1, 28));
        m.insert(1551, NaiveDate::from_ymd(1551, 2, 16));
        m.insert(1552, NaiveDate::from_ymd(1552, 2, 5));
        m.insert(1553, NaiveDate::from_ymd(1553, 1, 24));
        m.insert(1554, NaiveDate::from_ymd(1554, 2, 12));
        m.insert(1555, NaiveDate::from_ymd(1555, 2, 2));
        m.insert(1556, NaiveDate::from_ymd(1556, 2, 21));
        m.insert(1557, NaiveDate::from_ymd(1557, 2, 10));
        m.insert(1558, NaiveDate::from_ymd(1558, 1, 30));
        m.insert(1559, NaiveDate::from_ymd(1559, 2, 18));
        m.insert(1560, NaiveDate::from_ymd(1560, 2, 7));
        m.insert(1561, NaiveDate::from_ymd(1561, 1, 26));
        m.insert(1562, NaiveDate::from_ymd(1562, 2, 14));
        m.insert(1563, NaiveDate::from_ymd(1563, 2, 3));
        m.insert(1564, NaiveDate::from_ymd(1564, 2, 23));
        m.insert(1565, NaiveDate::from_ymd(1565, 2, 11));
        m.insert(1566, NaiveDate::from_ymd(1566, 2, 1));
        m.insert(1567, NaiveDate::from_ymd(1567, 2, 19));
        m.insert(1568, NaiveDate::from_ymd(1568, 2, 8));
        m.insert(1569, NaiveDate::from_ymd(1569, 1, 27));
        m.insert(1570, NaiveDate::from_ymd(1570, 2, 15));
        m.insert(1571, NaiveDate::from_ymd(1571, 2, 5));
        m.insert(1572, NaiveDate::from_ymd(1572, 1, 25));
        m.insert(1573, NaiveDate::from_ymd(1573, 2, 13));
        m.insert(1574, NaiveDate::from_ymd(1574, 2, 2));
        m.insert(1575, NaiveDate::from_ymd(1575, 2, 21));
        m.insert(1576, NaiveDate::from_ymd(1576, 2, 10));
        m.insert(1577, NaiveDate::from_ymd(1577, 1, 29));
        m.insert(1578, NaiveDate::from_ymd(1578, 2, 17));
        m.insert(1579, NaiveDate::from_ymd(1579, 2, 6));
        m.insert(1580, NaiveDate::from_ymd(1580, 1, 27));
        m.insert(1581, NaiveDate::from_ymd(1581, 2, 14));
        m.insert(1582, NaiveDate::from_ymd(1582, 2, 3));
        m.insert(1583, NaiveDate::from_ymd(1583, 1, 24));
        m.insert(1584, NaiveDate::from_ymd(1584, 2, 12));
        m.insert(1585, NaiveDate::from_ymd(1585, 1, 31));
        m.insert(1586, NaiveDate::from_ymd(1586, 2, 19));
        m.insert(1587, NaiveDate::from_ymd(1587, 2, 8));
        m.insert(1588, NaiveDate::from_ymd(1588, 1, 28));
        m.insert(1589, NaiveDate::from_ymd(1589, 2, 15));
        m.insert(1590, NaiveDate::from_ymd(1590, 2, 5));
        m.insert(1591, NaiveDate::from_ymd(1591, 1, 25));
        m.insert(1592, NaiveDate::from_ymd(1592, 2, 13));
        m.insert(1593, NaiveDate::from_ymd(1593, 2, 2));
        m.insert(1594, NaiveDate::from_ymd(1594, 2, 20));
        m.insert(1595, NaiveDate::from_ymd(1595, 2, 9));
        m.insert(1596, NaiveDate::from_ymd(1596, 1, 30));
        m.insert(1597, NaiveDate::from_ymd(1597, 2, 17));
        m.insert(1598, NaiveDate::from_ymd(1598, 2, 6));
        m.insert(1599, NaiveDate::from_ymd(1599, 1, 27));
        m.insert(1600, NaiveDate::from_ymd(1600, 2, 15));
        m.insert(1601, NaiveDate::from_ymd(1601, 2, 3));
        m.insert(1602, NaiveDate::from_ymd(1602, 2, 22));
        m.insert(1603, NaiveDate::from_ymd(1603, 2, 11));
        m.insert(1604, NaiveDate::from_ymd(1604, 1, 31));
        m.insert(1605, NaiveDate::from_ymd(1605, 2, 18));
        m.insert(1606, NaiveDate::from_ymd(1606, 2, 7));
        m.insert(1607, NaiveDate::from_ymd(1607, 1, 28));
        m.insert(1608, NaiveDate::from_ymd(1608, 2, 16));
        m.insert(1609, NaiveDate::from_ymd(1609, 2, 5));
        m.insert(1610, NaiveDate::from_ymd(1610, 1, 25));
        m.insert(1611, NaiveDate::from_ymd(1611, 2, 13));
        m.insert(1612, NaiveDate::from_ymd(1612, 2, 2));
        m.insert(1613, NaiveDate::from_ymd(1613, 2, 20));
        m.insert(1614, NaiveDate::from_ymd(1614, 2, 9));
        m.insert(1615, NaiveDate::from_ymd(1615, 1, 29));
        m.insert(1616, NaiveDate::from_ymd(1616, 2, 17));
        m.insert(1617, NaiveDate::from_ymd(1617, 2, 6));
        m.insert(1618, NaiveDate::from_ymd(1618, 1, 27));
        m.insert(1619, NaiveDate::from_ymd(1619, 2, 15));
        m.insert(1620, NaiveDate::from_ymd(1620, 2, 4));
        m.insert(1621, NaiveDate::from_ymd(1621, 2, 22));
        m.insert(1622, NaiveDate::from_ymd(1622, 2, 11));
        m.insert(1623, NaiveDate::from_ymd(1623, 1, 31));
        m.insert(1624, NaiveDate::from_ymd(1624, 2, 19));
        m.insert(1625, NaiveDate::from_ymd(1625, 2, 7));
        m.insert(1626, NaiveDate::from_ymd(1626, 1, 28));
        m.insert(1627, NaiveDate::from_ymd(1627, 2, 16));
        m.insert(1628, NaiveDate::from_ymd(1628, 2, 5));
        m.insert(1629, NaiveDate::from_ymd(1629, 1, 25));
        m.insert(1630, NaiveDate::from_ymd(1630, 2, 12));
        m.insert(1631, NaiveDate::from_ymd(1631, 2, 1));
        m.insert(1632, NaiveDate::from_ymd(1632, 2, 20));
        m.insert(1633, NaiveDate::from_ymd(1633, 2, 9));
        m.insert(1634, NaiveDate::from_ymd(1634, 1, 29));
        m.insert(1635, NaiveDate::from_ymd(1635, 2, 18));
        m.insert(1636, NaiveDate::from_ymd(1636, 2, 7));
        m.insert(1637, NaiveDate::from_ymd(1637, 1, 26));
        m.insert(1638, NaiveDate::from_ymd(1638, 2, 14));
        m.insert(1639, NaiveDate::from_ymd(1639, 2, 3));
        m.insert(1640, NaiveDate::from_ymd(1640, 2, 22));
        m.insert(1641, NaiveDate::from_ymd(1641, 2, 10));
        m.insert(1642, NaiveDate::from_ymd(1642, 1, 31));
        m.insert(1643, NaiveDate::from_ymd(1643, 2, 19));
        m.insert(1644, NaiveDate::from_ymd(1644, 2, 8));
        m.insert(1645, NaiveDate::from_ymd(1645, 1, 28));
        m.insert(1646, NaiveDate::from_ymd(1646, 2, 16));
        m.insert(1647, NaiveDate::from_ymd(1647, 2, 5));
        m.insert(1648, NaiveDate::from_ymd(1648, 1, 25));
        m.insert(1649, NaiveDate::from_ymd(1649, 2, 12));
        m.insert(1650, NaiveDate::from_ymd(1650, 2, 1));
        m.insert(1651, NaiveDate::from_ymd(1651, 2, 20));
        m.insert(1652, NaiveDate::from_ymd(1652, 2, 10));
        m.insert(1653, NaiveDate::from_ymd(1653, 1, 29));
        m.insert(1654, NaiveDate::from_ymd(1654, 2, 17));
        m.insert(1655, NaiveDate::from_ymd(1655, 2, 7));
        m.insert(1656, NaiveDate::from_ymd(1656, 1, 27));
        m.insert(1657, NaiveDate::from_ymd(1657, 2, 13));
        m.insert(1658, NaiveDate::from_ymd(1658, 2, 3));
        m.insert(1659, NaiveDate::from_ymd(1659, 2, 22));
        m.insert(1660, NaiveDate::from_ymd(1660, 2, 11));
        m.insert(1661, NaiveDate::from_ymd(1661, 1, 31));
        m.insert(1662, NaiveDate::from_ymd(1662, 2, 19));
        m.insert(1663, NaiveDate::from_ymd(1663, 2, 8));
        m.insert(1664, NaiveDate::from_ymd(1664, 1, 28));
        m.insert(1665, NaiveDate::from_ymd(1665, 2, 15));
        m.insert(1666, NaiveDate::from_ymd(1666, 2, 4));
        m.insert(1667, NaiveDate::from_ymd(1667, 1, 24));
        m.insert(1668, NaiveDate::from_ymd(1668, 2, 12));
        m.insert(1669, NaiveDate::from_ymd(1669, 2, 1));
        m.insert(1670, NaiveDate::from_ymd(1670, 2, 20));
        m.insert(1671, NaiveDate::from_ymd(1671, 2, 10));
        m.insert(1672, NaiveDate::from_ymd(1672, 1, 30));
        m.insert(1673, NaiveDate::from_ymd(1673, 2, 17));
        m.insert(1674, NaiveDate::from_ymd(1674, 2, 6));
        m.insert(1675, NaiveDate::from_ymd(1675, 1, 26));
        m.insert(1676, NaiveDate::from_ymd(1676, 2, 14));
        m.insert(1677, NaiveDate::from_ymd(1677, 2, 2));
        m.insert(1678, NaiveDate::from_ymd(1678, 2, 21));
        m.insert(1679, NaiveDate::from_ymd(1679, 2, 11));
        m.insert(1680, NaiveDate::from_ymd(1680, 2, 1));
        m.insert(1681, NaiveDate::from_ymd(1681, 2, 19));
        m.insert(1682, NaiveDate::from_ymd(1682, 2, 8));
        m.insert(1683, NaiveDate::from_ymd(1683, 1, 28));
        m.insert(1684, NaiveDate::from_ymd(1684, 2, 16));
        m.insert(1685, NaiveDate::from_ymd(1685, 2, 4));
        m.insert(1686, NaiveDate::from_ymd(1686, 1, 24));
        m.insert(1687, NaiveDate::from_ymd(1687, 2, 12));
        m.insert(1688, NaiveDate::from_ymd(1688, 2, 2));
        m.insert(1689, NaiveDate::from_ymd(1689, 1, 21));
        m.insert(1690, NaiveDate::from_ymd(1690, 2, 9));
        m.insert(1691, NaiveDate::from_ymd(1691, 1, 29));
        m.insert(1692, NaiveDate::from_ymd(1692, 2, 17));
        m.insert(1693, NaiveDate::from_ymd(1693, 2, 5));
        m.insert(1694, NaiveDate::from_ymd(1694, 1, 25));
        m.insert(1695, NaiveDate::from_ymd(1695, 2, 13));
        m.insert(1696, NaiveDate::from_ymd(1696, 2, 3));
        m.insert(1697, NaiveDate::from_ymd(1697, 1, 23));
        m.insert(1698, NaiveDate::from_ymd(1698, 2, 11));
        m.insert(1699, NaiveDate::from_ymd(1699, 1, 31));
        m.insert(1700, NaiveDate::from_ymd(1700, 2, 19));
        m.insert(1701, NaiveDate::from_ymd(1701, 2, 8));
        m.insert(1702, NaiveDate::from_ymd(1702, 1, 28));
        m.insert(1703, NaiveDate::from_ymd(1703, 2, 16));
        m.insert(1704, NaiveDate::from_ymd(1704, 2, 5));
        m.insert(1705, NaiveDate::from_ymd(1705, 1, 25));
        m.insert(1706, NaiveDate::from_ymd(1706, 2, 13));
        m.insert(1707, NaiveDate::from_ymd(1707, 2, 3));
        m.insert(1708, NaiveDate::from_ymd(1708, 1, 23));
        m.insert(1709, NaiveDate::from_ymd(1709, 2, 10));
        m.insert(1710, NaiveDate::from_ymd(1710, 1, 30));
        m.insert(1711, NaiveDate::from_ymd(1711, 2, 17));
        m.insert(1712, NaiveDate::from_ymd(1712, 2, 7));
        m.insert(1713, NaiveDate::from_ymd(1713, 1, 26));
        m.insert(1714, NaiveDate::from_ymd(1714, 2, 15));
        m.insert(1715, NaiveDate::from_ymd(1715, 2, 4));
        m.insert(1716, NaiveDate::from_ymd(1716, 1, 25));
        m.insert(1717, NaiveDate::from_ymd(1717, 2, 11));
        m.insert(1718, NaiveDate::from_ymd(1718, 1, 31));
        m.insert(1719, NaiveDate::from_ymd(1719, 2, 19));
        m.insert(1720, NaiveDate::from_ymd(1720, 2, 8));
        m.insert(1721, NaiveDate::from_ymd(1721, 1, 28));
        m.insert(1722, NaiveDate::from_ymd(1722, 2, 16));
        m.insert(1723, NaiveDate::from_ymd(1723, 2, 5));
        m.insert(1724, NaiveDate::from_ymd(1724, 1, 26));
        m.insert(1725, NaiveDate::from_ymd(1725, 2, 13));
        m.insert(1726, NaiveDate::from_ymd(1726, 2, 2));
        m.insert(1727, NaiveDate::from_ymd(1727, 1, 22));
        m.insert(1728, NaiveDate::from_ymd(1728, 2, 10));
        m.insert(1729, NaiveDate::from_ymd(1729, 1, 29));
        m.insert(1730, NaiveDate::from_ymd(1730, 2, 17));
        m.insert(1731, NaiveDate::from_ymd(1731, 2, 7));
        m.insert(1732, NaiveDate::from_ymd(1732, 1, 27));
        m.insert(1733, NaiveDate::from_ymd(1733, 2, 14));
        m.insert(1734, NaiveDate::from_ymd(1734, 2, 4));
        m.insert(1735, NaiveDate::from_ymd(1735, 1, 24));
        m.insert(1736, NaiveDate::from_ymd(1736, 2, 12));
        m.insert(1737, NaiveDate::from_ymd(1737, 1, 31));
        m.insert(1738, NaiveDate::from_ymd(1738, 2, 19));
        m.insert(1739, NaiveDate::from_ymd(1739, 2, 8));
        m.insert(1740, NaiveDate::from_ymd(1740, 1, 29));
        m.insert(1741, NaiveDate::from_ymd(1741, 2, 16));
        m.insert(1742, NaiveDate::from_ymd(1742, 2, 5));
        m.insert(1743, NaiveDate::from_ymd(1743, 1, 26));
        m.insert(1744, NaiveDate::from_ymd(1744, 2, 14));
        m.insert(1745, NaiveDate::from_ymd(1745, 2, 1));
        m.insert(1746, NaiveDate::from_ymd(1746, 2, 20));
        m.insert(1747, NaiveDate::from_ymd(1747, 2, 10));
        m.insert(1748, NaiveDate::from_ymd(1748, 1, 30));
        m.insert(1749, NaiveDate::from_ymd(1749, 2, 17));
        m.insert(1750, NaiveDate::from_ymd(1750, 2, 7));
        m.insert(1751, NaiveDate::from_ymd(1751, 1, 27));
        m.insert(1752, NaiveDate::from_ymd(1752, 2, 15));
        m.insert(1753, NaiveDate::from_ymd(1753, 2, 3));
        m.insert(1754, NaiveDate::from_ymd(1754, 1, 23));
        m.insert(1755, NaiveDate::from_ymd(1755, 2, 11));
        m.insert(1756, NaiveDate::from_ymd(1756, 1, 31));
        m.insert(1757, NaiveDate::from_ymd(1757, 2, 18));
        m.insert(1758, NaiveDate::from_ymd(1758, 2, 8));
        m.insert(1759, NaiveDate::from_ymd(1759, 1, 29));
        m.insert(1760, NaiveDate::from_ymd(1760, 2, 17));
        m.insert(1761, NaiveDate::from_ymd(1761, 2, 5));
        m.insert(1762, NaiveDate::from_ymd(1762, 1, 25));
        m.insert(1763, NaiveDate::from_ymd(1763, 2, 13));
        m.insert(1764, NaiveDate::from_ymd(1764, 2, 2));
        m.insert(1765, NaiveDate::from_ymd(1765, 2, 20));
        m.insert(1766, NaiveDate::from_ymd(1766, 2, 9));
        m.insert(1767, NaiveDate::from_ymd(1767, 1, 30));
        m.insert(1768, NaiveDate::from_ymd(1768, 2, 18));
        m.insert(1769, NaiveDate::from_ymd(1769, 2, 7));
        m.insert(1770, NaiveDate::from_ymd(1770, 1, 27));
        m.insert(1771, NaiveDate::from_ymd(1771, 2, 15));
        m.insert(1772, NaiveDate::from_ymd(1772, 2, 4));
        m.insert(1773, NaiveDate::from_ymd(1773, 1, 23));
        m.insert(1774, NaiveDate::from_ymd(1774, 2, 11));
        m.insert(1775, NaiveDate::from_ymd(1775, 1, 31));
        m.insert(1776, NaiveDate::from_ymd(1776, 2, 19));
        m.insert(1777, NaiveDate::from_ymd(1777, 2, 8));
        m.insert(1778, NaiveDate::from_ymd(1778, 1, 28));
        m.insert(1779, NaiveDate::from_ymd(1779, 2, 16));
        m.insert(1780, NaiveDate::from_ymd(1780, 2, 5));
        m.insert(1781, NaiveDate::from_ymd(1781, 1, 24));
        m.insert(1782, NaiveDate::from_ymd(1782, 2, 12));
        m.insert(1783, NaiveDate::from_ymd(1783, 2, 2));
        m.insert(1784, NaiveDate::from_ymd(1784, 1, 22));
        m.insert(1785, NaiveDate::from_ymd(1785, 2, 9));
        m.insert(1786, NaiveDate::from_ymd(1786, 1, 30));
        m.insert(1787, NaiveDate::from_ymd(1787, 2, 18));
        m.insert(1788, NaiveDate::from_ymd(1788, 2, 7));
        m.insert(1789, NaiveDate::from_ymd(1789, 1, 26));
        m.insert(1790, NaiveDate::from_ymd(1790, 2, 14));
        m.insert(1791, NaiveDate::from_ymd(1791, 2, 3));
        m.insert(1792, NaiveDate::from_ymd(1792, 1, 24));
        m.insert(1793, NaiveDate::from_ymd(1793, 2, 11));
        m.insert(1794, NaiveDate::from_ymd(1794, 1, 31));
        m.insert(1795, NaiveDate::from_ymd(1795, 2, 19));
        m.insert(1796, NaiveDate::from_ymd(1796, 2, 9));
        m.insert(1797, NaiveDate::from_ymd(1797, 1, 28));
        m.insert(1798, NaiveDate::from_ymd(1798, 2, 16));
        m.insert(1799, NaiveDate::from_ymd(1799, 2, 5));
        m.insert(1800, NaiveDate::from_ymd(1800, 1, 25));
        m.insert(1801, NaiveDate::from_ymd(1801, 2, 13));
        m.insert(1802, NaiveDate::from_ymd(1802, 2, 3));
        m.insert(1803, NaiveDate::from_ymd(1803, 1, 23));
        m.insert(1804, NaiveDate::from_ymd(1804, 2, 11));
        m.insert(1805, NaiveDate::from_ymd(1805, 1, 31));
        m.insert(1806, NaiveDate::from_ymd(1806, 2, 18));
        m.insert(1807, NaiveDate::from_ymd(1807, 2, 7));
        m.insert(1808, NaiveDate::from_ymd(1808, 1, 28));
        m.insert(1809, NaiveDate::from_ymd(1809, 2, 14));
        m.insert(1810, NaiveDate::from_ymd(1810, 2, 4));
        m.insert(1811, NaiveDate::from_ymd(1811, 1, 25));
        m.insert(1812, NaiveDate::from_ymd(1812, 2, 13));
        m.insert(1813, NaiveDate::from_ymd(1813, 2, 1));
        m.insert(1814, NaiveDate::from_ymd(1814, 2, 20));
        m.insert(1815, NaiveDate::from_ymd(1815, 2, 9));
        m.insert(1816, NaiveDate::from_ymd(1816, 1, 29));
        m.insert(1817, NaiveDate::from_ymd(1817, 2, 16));
        m.insert(1818, NaiveDate::from_ymd(1818, 2, 5));
        m.insert(1819, NaiveDate::from_ymd(1819, 1, 26));
        m.insert(1820, NaiveDate::from_ymd(1820, 2, 14));
        m.insert(1821, NaiveDate::from_ymd(1821, 2, 3));
        m.insert(1822, NaiveDate::from_ymd(1822, 1, 23));
        m.insert(1823, NaiveDate::from_ymd(1823, 2, 11));
        m.insert(1824, NaiveDate::from_ymd(1824, 1, 31));
        m.insert(1825, NaiveDate::from_ymd(1825, 2, 18));
        m.insert(1826, NaiveDate::from_ymd(1826, 2, 7));
        m.insert(1827, NaiveDate::from_ymd(1827, 1, 27));
        m.insert(1828, NaiveDate::from_ymd(1828, 2, 15));
        m.insert(1829, NaiveDate::from_ymd(1829, 2, 4));
        m.insert(1830, NaiveDate::from_ymd(1830, 1, 25));
        m.insert(1831, NaiveDate::from_ymd(1831, 2, 13));
        m.insert(1832, NaiveDate::from_ymd(1832, 2, 2));
        m.insert(1833, NaiveDate::from_ymd(1833, 2, 20));
        m.insert(1834, NaiveDate::from_ymd(1834, 2, 9));
        m.insert(1835, NaiveDate::from_ymd(1835, 1, 29));
        m.insert(1836, NaiveDate::from_ymd(1836, 2, 17));
        m.insert(1837, NaiveDate::from_ymd(1837, 2, 5));
        m.insert(1838, NaiveDate::from_ymd(1838, 1, 26));
        m.insert(1839, NaiveDate::from_ymd(1839, 2, 14));
        m.insert(1840, NaiveDate::from_ymd(1840, 2, 3));
        m.insert(1841, NaiveDate::from_ymd(1841, 1, 23));
        m.insert(1842, NaiveDate::from_ymd(1842, 2, 10));
        m.insert(1843, NaiveDate::from_ymd(1843, 1, 30));
        m.insert(1844, NaiveDate::from_ymd(1844, 2, 18));
        m.insert(1845, NaiveDate::from_ymd(1845, 2, 7));
        m.insert(1846, NaiveDate::from_ymd(1846, 1, 27));
        m.insert(1847, NaiveDate::from_ymd(1847, 2, 15));
        m.insert(1848, NaiveDate::from_ymd(1848, 2, 5));
        m.insert(1849, NaiveDate::from_ymd(1849, 1, 24));
        m.insert(1850, NaiveDate::from_ymd(1850, 2, 12));
        m.insert(1851, NaiveDate::from_ymd(1851, 2, 1));
        m.insert(1852, NaiveDate::from_ymd(1852, 1, 21));
        m.insert(1853, NaiveDate::from_ymd(1853, 2, 8));
        m.insert(1854, NaiveDate::from_ymd(1854, 1, 29));
        m.insert(1855, NaiveDate::from_ymd(1855, 2, 17));
        m.insert(1856, NaiveDate::from_ymd(1856, 2, 6));
        m.insert(1857, NaiveDate::from_ymd(1857, 1, 26));
        m.insert(1858, NaiveDate::from_ymd(1858, 2, 14));
        m.insert(1859, NaiveDate::from_ymd(1859, 2, 3));
        m.insert(1860, NaiveDate::from_ymd(1860, 1, 23));
        m.insert(1861, NaiveDate::from_ymd(1861, 2, 10));
        m.insert(1862, NaiveDate::from_ymd(1862, 1, 30));
        m.insert(1863, NaiveDate::from_ymd(1863, 2, 18));
        m.insert(1864, NaiveDate::from_ymd(1864, 2, 8));
        m.insert(1865, NaiveDate::from_ymd(1865, 1, 27));
        m.insert(1866, NaiveDate::from_ymd(1866, 2, 15));
        m.insert(1867, NaiveDate::from_ymd(1867, 2, 5));
        m.insert(1868, NaiveDate::from_ymd(1868, 1, 25));
        m.insert(1869, NaiveDate::from_ymd(1869, 2, 11));
        m.insert(1870, NaiveDate::from_ymd(1870, 2, 1));
        m.insert(1871, NaiveDate::from_ymd(1871, 2, 19));
        m.insert(1872, NaiveDate::from_ymd(1872, 2, 9));
        m
    };

    static ref ERAS: HashMap<&'static str, Era> = {
        let mut m = HashMap::new();
        //Asuka period
        m.insert("大化", Era::new("Taika", "大化", "たいか", "Taika", "Taika", NaiveDate::from_ymd(645, 7, 20), NaiveDate::from_ymd(650, 3, 25), 6, Court::Unified, None, None));
        m.insert("白雉", Era::new("Hakuchi", "白雉", "はくち", "Hakuchi", "Hakuti", NaiveDate::from_ymd(650, 3, 25), NaiveDate::from_ymd(654, 11, 27), 5, Court::Unified, None, None));
        m.insert("朱鳥", Era::new("Shuchou", "朱鳥", "しゅちょう", "Shuchō", "Syutyô", NaiveDate::from_ymd(686, 8, 17), NaiveDate::from_ymd(686, 10, 4), 1, Court::Unified, None, None));
        m.insert("大宝", Era::new("Taihou", "大宝", "たいほう", "Taihō", "Taihô", NaiveDate::from_ymd(701, 5, 7), NaiveDate::from_ymd(704, 6, 20), 4, Court::Unified, None, None));
        m.insert("慶雲", Era::new("Keiun", "慶雲", "けいうん", "Keiun", "Keiun", NaiveDate::from_ymd(704, 6, 20), NaiveDate::from_ymd(708, 2, 11), 5, Court::Unified, None, None));
        m.insert("和銅", Era::new("Wadou", "和銅", "わどう", "Wadō", "Wadô", NaiveDate::from_ymd(708, 2, 11), NaiveDate::from_ymd(715, 10, 7), 8, Court::Unified, None, None));
        //Nara period
        m.insert("霊亀", Era::new("Reiki", "霊亀", "れいき", "Reiki", "Reiki", NaiveDate::from_ymd(715, 10, 7), NaiveDate::from_ymd(717, 12, 28), 3, Court::Unified, None, None));
        m.insert("養老", Era::new("Yourou", "養老", "ようろう", "Yōrō", "Yôrô", NaiveDate::from_ymd(717, 12, 28), NaiveDate::from_ymd(724, 3, 27), 8, Court::Unified, None, None));
        m.insert("神亀", Era::new("Jinki", "神亀", "じんき", "Jinki", "Zinki", NaiveDate::from_ymd(724, 3, 27), NaiveDate::from_ymd(729, 9, 6), 6, Court::Unified, None, None));
        m.insert("天平", Era::new("Tenpyou", "天平", "てんぴょう", "Tenpyō", "Tenpyô", NaiveDate::from_ymd(729, 9, 6), NaiveDate::from_ymd(749, 5, 9), 21, Court::Unified, None, None));
        m.insert("天平感宝", Era::new("Tenpyou-Kanpou", "天平感宝", "てんぴょうかんぽう", "Tenpyō-Kanpō", "Tenpyô-Kanpô", NaiveDate::from_ymd(749, 5, 9), NaiveDate::from_ymd(749, 8, 23), 1, Court::Unified, None, None));
        m.insert("天平勝宝", Era::new("Tenpyou", "天平勝宝", "てんぴょうしょうほう", "Tenpyō-Shōhō", "Tenpyô-Syôhô", NaiveDate::from_ymd(749, 8, 23), NaiveDate::from_ymd(757, 9, 10), 9, Court::Unified, None, None));
        m.insert("天平宝字", Era::new("Tenpyou-Houji", "天平宝字", "てんぴょうほうじ", "Tenpyō-Hōji", "Tenpyô-Hôji", NaiveDate::from_ymd(757, 9, 10), NaiveDate::from_ymd(765, 2, 5), 9, Court::Unified, None, None));
        m.insert("天平神護", Era::new("Tenpyou-Jingo", "天平神護", "てんぴょう", "Tenpyō-Jingo", "Tenpyô-Zingo", NaiveDate::from_ymd(765, 2, 5), NaiveDate::from_ymd(767, 9, 17), 3, Court::Unified, None, None));
        m.insert("神護景雲", Era::new("Jingo-Keiun", "神護景雲", "じんごけいうん", "Jingo-Keiun", "Zingo-Keiun", NaiveDate::from_ymd(767, 9, 17), NaiveDate::from_ymd(770, 10, 27), 4, Court::Unified, None, None));
        m.insert("宝亀", Era::new("Houki", "宝亀", "ほうき", "Hōki", "Hôki", NaiveDate::from_ymd(770, 10, 27), NaiveDate::from_ymd(781, 2, 3), 12, Court::Unified, None, None));
        m.insert("天応", Era::new("Ten'ou", "天応", "てんおう", "Ten'ō", "Ten'ô", NaiveDate::from_ymd(781, 2, 3), NaiveDate::from_ymd(782, 10, 4), 2, Court::Unified, None, None));
        m.insert("延暦", Era::new("Enryaku", "延暦", "えんりゃく", "Enryaku", "Enryaku", NaiveDate::from_ymd(782, 10, 4), NaiveDate::from_ymd(806, 6, 12), 25, Court::Unified, None, None));
        //Heian period
        m.insert("大同", Era::new("Daidou", "大同", "だいどう", "Daidō", "Daidô", NaiveDate::from_ymd(806, 6, 12), NaiveDate::from_ymd(810, 10, 24), 5, Court::Unified, None, None));
        m.insert("弘仁", Era::new("Kounin", "弘仁", "こうにん", "Kōnin", "Kônin", NaiveDate::from_ymd(810, 10, 24), NaiveDate::from_ymd(824, 2, 12), 15, Court::Unified, None, None));
        m.insert("天長", Era::new("Tenchou", "天長", "てんちょう", "Tenchō", "Tentyô", NaiveDate::from_ymd(824, 2, 12), NaiveDate::from_ymd(834, 2, 18), 11, Court::Unified, None, None));
        m.insert("承和", Era::new("Jouwa", "承和", "じょうわ", "Jōwa", "Zyôwa", NaiveDate::from_ymd(834, 2, 18), NaiveDate::from_ymd(848, 7, 20), 15, Court::Unified, None, None));
        m.insert("嘉祥", Era::new("Kashou", "嘉祥", "かしょう", "Kashō", "Kasyô", NaiveDate::from_ymd(848, 7, 20), NaiveDate::from_ymd(851, 6, 5), 4, Court::Unified, None, None));
        m.insert("仁寿", Era::new("Ninju", "仁寿", "にんじゅ", "Ninju", "Ninzyu", NaiveDate::from_ymd(851, 6, 5), NaiveDate::from_ymd(854, 12, 27), 4, Court::Unified, None, None));
        m.insert("斉衡", Era::new("Saikou", "斉衡", "さいこう", "Saikō", "Saikô", NaiveDate::from_ymd(854, 12, 27), NaiveDate::from_ymd(857, 3, 24), 4, Court::Unified, None, None));
        m.insert("天安", Era::new("Ten'an", "天安", "てんあん", "Ten'an", "Ten'an", NaiveDate::from_ymd(857, 3, 24), NaiveDate::from_ymd(859, 5, 24), 3, Court::Unified, None, None));
        m.insert("貞観", Era::new("Jougan", "貞観", "じょうがん", "Jōgan", "Zyôgan", NaiveDate::from_ymd(859, 5, 24), NaiveDate::from_ymd(877, 6, 5), 19, Court::Unified, None, None));
        m.insert("元慶", Era::new("Gangyou", "元慶", "がんぎょう", "Gangyō", "Gangyô", NaiveDate::from_ymd(877, 6, 5), NaiveDate::from_ymd(885, 3, 15), 9, Court::Unified, None, None));
        m.insert("仁和", Era::new("Nin'na", "仁和", "にんな", "Ninna", "Ninna", NaiveDate::from_ymd(885, 3, 15), NaiveDate::from_ymd(889, 6, 3), 5, Court::Unified, None, None));
        m.insert("寛平", Era::new("Kanpyou", "寛平", "かんぴょう", "Kanpyō", "Kanpyô", NaiveDate::from_ymd(889, 6, 3), NaiveDate::from_ymd(898, 5, 24), 10, Court::Unified, None, None));	
        m.insert("昌泰", Era::new("Shoutai", "昌泰", "しょうたい", "Shōtai", "Syôtai", NaiveDate::from_ymd(898, 5, 24), NaiveDate::from_ymd(901, 9, 5), 4, Court::Unified, None, None));
        m.insert("延喜", Era::new("Engi", "延喜", "えんぎ", "Engi", "Engi", NaiveDate::from_ymd(901, 9, 5), NaiveDate::from_ymd(923, 6, 3), 23, Court::Unified, None, None));
        m.insert("延長", Era::new("Enchou", "延長", "えんちょう", "Enchō", "Entyô", NaiveDate::from_ymd(923, 6, 3), NaiveDate::from_ymd(931, 5, 21), 9, Court::Unified, None, None));
        m.insert("承平", Era::new("Jouhei", "承平", "じょうへい", "Jōhei", "Zyôhei", NaiveDate::from_ymd(931, 5, 21), NaiveDate::from_ymd(938, 6, 27), 8, Court::Unified, None, None));	
        m.insert("天慶", Era::new("Tengyou", "天慶", "てんぎょう", "Tengyō", "Tengyô", NaiveDate::from_ymd(938, 6, 27), NaiveDate::from_ymd(947, 5, 20), 10, Court::Unified, None, None));
        m.insert("天暦", Era::new("Tenryaku", "天暦", "てんりゃく", "Tenryaku", "Tenryaku", NaiveDate::from_ymd(947, 5, 20), NaiveDate::from_ymd(957, 11, 26), 11, Court::Unified, None, None));
        m.insert("天徳", Era::new("Tentoku", "天徳", "てんとく", "Tentoku", "Tentoku", NaiveDate::from_ymd(957, 11, 26), NaiveDate::from_ymd(961, 3, 10), 5, Court::Unified, None, None));	
        m.insert("応和", Era::new("Ouwa", "応和", "おうわ", "Ōwa", "Ôwa", NaiveDate::from_ymd(961, 3, 10), NaiveDate::from_ymd(964, 8, 24), 4, Court::Unified, None, None));	
        m.insert("康保", Era::new("Kouhou", "康保", "こうほう", "Kōhō", "Kôhô", NaiveDate::from_ymd(964, 8, 24), NaiveDate::from_ymd(968, 9, 13), 5, Court::Unified, None, None));
        m.insert("安和", Era::new("An'na", "安和", "あんな", "Anna", "Anna", NaiveDate::from_ymd(968, 9, 13), NaiveDate::from_ymd(970, 5, 8), 3, Court::Unified, None, None));
        m.insert("天禄", Era::new("Tenroku", "天禄", "てんろく", "Tenroku", "Tenroku", NaiveDate::from_ymd(970, 5, 8), NaiveDate::from_ymd(974, 1, 21), 4, Court::Unified, None, None));
        m.insert("天延", Era::new("Ten'en", "天延", "てんえん", "Ten'en", "Ten'en", NaiveDate::from_ymd(974, 1, 21), NaiveDate::from_ymd(976, 8, 16), 4, Court::Unified, None, None));	
        m.insert("貞元", Era::new("Jougen (Heian)", "貞元", "じょうげん（へいあんじだい）", "Jōgen (Heian)", "Zyôgen (Heian)", NaiveDate::from_ymd(976, 8, 16), NaiveDate::from_ymd(979, 1, 5), 3, Court::Unified, None, None));
        m.insert("天元", Era::new("Tengen", "天元", "てんげん", "Tengen", "Tengen", NaiveDate::from_ymd(979, 1, 5), NaiveDate::from_ymd(983, 6, 3), 6, Court::Unified, None, None));
        m.insert("永観", Era::new("Eikan", "永観", "えいかん", "Eikan", "Eikan", NaiveDate::from_ymd(983, 6, 3), NaiveDate::from_ymd(985, 5, 24), 3, Court::Unified, None, None));
        m.insert("寛和", Era::new("Kan'na", "寛和", "かんな", "Kanna", "Kanna", NaiveDate::from_ymd(985, 5, 24), NaiveDate::from_ymd(987, 5, 10), 3, Court::Unified, None, None));
        m.insert("永延", Era::new("Eien", "永延", "えいえん", "Eien", "Eien", NaiveDate::from_ymd(987, 5, 10), NaiveDate::from_ymd(989, 9, 15), 3, Court::Unified, None, None));	
        m.insert("永祚", Era::new("Eiso", "永祚", "えいそ", "Eiso", "Eiso", NaiveDate::from_ymd(989, 9, 15), NaiveDate::from_ymd(990, 12, 1), 2, Court::Unified, None, None));
        m.insert("正暦", Era::new("Shouryaku", "正暦", "しょうりゃく", "Shōryaku", "Syôryaku", NaiveDate::from_ymd(990, 12, 1), NaiveDate::from_ymd(995, 3, 30), 6, Court::Unified, None, None));
        m.insert("長徳", Era::new("Choutoku", "長徳", "ちょうとく", "Chōtoku", "Tyôtoku", NaiveDate::from_ymd(995, 3, 30), NaiveDate::from_ymd(999, 2, 6), 5, Court::Unified, None, None));
        m.insert("長保", Era::new("Chouhou", "長保", "ちょうほう", "Chōhō", "Tyôhô", NaiveDate::from_ymd(999, 2, 6), NaiveDate::from_ymd(1004, 8, 14), 6, Court::Unified, None, None));
        m.insert("寛弘", Era::new("Kankou", "寛弘", "かんこう", "Kankō", "Kankô", NaiveDate::from_ymd(1004, 8, 14), NaiveDate::from_ymd(1013, 2, 4), 9, Court::Unified, None, None));
        m.insert("長和", Era::new("Chouwa", "長和", "ちょうわ", "Chōwa", "Tyôwa", NaiveDate::from_ymd(1013, 2, 4), NaiveDate::from_ymd(1017, 5, 27), 6, Court::Unified, None, None));
        m.insert("寛仁", Era::new("Kan'nin", "寛仁", "かんにん", "Kannin", "Kannin", NaiveDate::from_ymd(1017, 5, 27), NaiveDate::from_ymd(1021, 3, 23), 5, Court::Unified, None, None));
        m.insert("治安", Era::new("Jian", "治安", "じあん", "Jian", "Zian", NaiveDate::from_ymd(1021, 3, 23), NaiveDate::from_ymd(1024, 8, 25), 4, Court::Unified, None, None));
        m.insert("万寿", Era::new("Manju", "万寿", "まんじゅ", "Manju", "Manzyu", NaiveDate::from_ymd(1024, 8, 25), NaiveDate::from_ymd(1028, 8, 24), 5, Court::Unified, None, None));
        m.insert("長元", Era::new("Chougen", "長元", "ちょうげん", "Chōgen", "Tyôgen", NaiveDate::from_ymd(1028, 8, 24), NaiveDate::from_ymd(1037, 5, 15), 10, Court::Unified, None, None));
        m.insert("長暦", Era::new("Chouryaku", "長暦", "ちょうりゃく", "Chōryaku", "Tyôryaku", NaiveDate::from_ymd(1037, 5, 15), NaiveDate::from_ymd(1040, 12, 22), 4, Court::Unified, None, None));
        m.insert("長久", Era::new("Choukyuu", "長久", "ちょうきゅう", "Chōkyū", "Tyôkyû", NaiveDate::from_ymd(1040, 12, 22), NaiveDate::from_ymd(1044, 12, 22), 5, Court::Unified, None, None));
        m.insert("寛徳", Era::new("Kantoku", "寛徳", "かんとく", "Kantoku", "Kantoku", NaiveDate::from_ymd(1044, 12, 22), NaiveDate::from_ymd(1046, 5, 28), 3, Court::Unified, None, None));
        m.insert("永承", Era::new("Eijou", "永承", "えいじょう", "Eijō", "Eijyô", NaiveDate::from_ymd(1046, 5, 28), NaiveDate::from_ymd(1053, 2, 8), 8, Court::Unified, None, None));
        m.insert("天喜", Era::new("Tengi", "天喜", "てんぎ", "Tengi", "Tengi", NaiveDate::from_ymd(1053, 2, 8), NaiveDate::from_ymd(1058, 9, 25), 6, Court::Unified, None, None));
        m.insert("康平", Era::new("Kouhei", "康平", "こうへい", "Kōhei", "Kôhei", NaiveDate::from_ymd(1058, 9, 25), NaiveDate::from_ymd(1065, 9, 10), 8, Court::Unified, None, None));
        m.insert("治暦", Era::new("Jiryaku", "治暦", "じりゃく", "Jiryaku", "Ziryaku", NaiveDate::from_ymd(1065, 9, 10), NaiveDate::from_ymd(1069, 5, 12), 5, Court::Unified, None, None));
        m.insert("延久", Era::new("Enkyuu", "延久", "えんきゅう", "Enkyū", "Enkyû", NaiveDate::from_ymd(1069, 5, 12), NaiveDate::from_ymd(1074, 9, 22), 6, Court::Unified, None, None));
        m.insert("承保", Era::new("Jouhou", "承保", "じょうほう", "Jōhō", "Zyôhô", NaiveDate::from_ymd(1074, 9, 22), NaiveDate::from_ymd(1077, 12, 11), 4, Court::Unified, None, None));
        m.insert("承暦", Era::new("Jouryaku", "承暦", "じょうりゃく", "Jōryaku", "Zyôryaku", NaiveDate::from_ymd(1077, 12, 11), NaiveDate::from_ymd(1081, 3, 28), 5, Court::Unified, None, None));
        m.insert("永保", Era::new("Eihou", "永保", "えいほう", "Eihō", "Eihô", NaiveDate::from_ymd(1081, 3, 28), NaiveDate::from_ymd(1084, 3, 21), 4, Court::Unified, None, None));
        m.insert("応徳", Era::new("Outoku", "応徳", "おうとく", "Ōtoku", "Ôtoku", NaiveDate::from_ymd(1084, 3, 21), NaiveDate::from_ymd(1087, 5, 17), 4, Court::Unified, None, None));
        m.insert("寛治", Era::new("Kanji", "寛治", "かんじ", "Kanji", "Kanzi", NaiveDate::from_ymd(1087, 5, 17), NaiveDate::from_ymd(1095, 1, 29), 8, Court::Unified, None, None));
        m.insert("嘉保", Era::new("Kahou", "嘉保", "かほう", "Kahō", "Kahô", NaiveDate::from_ymd(1095, 1, 29), NaiveDate::from_ymd(1097, 1, 9), 3, Court::Unified, None, None));
        m.insert("永長", Era::new("Eichou", "永長", "えいちょう", "Eichō", "Eityô", NaiveDate::from_ymd(1097, 1, 9), NaiveDate::from_ymd(1098, 1, 2), 2, Court::Unified, None, None));
        m.insert("承徳", Era::new("Joutoku", "承徳", "じょうとく", "Jōtoku", "Zyôtoku", NaiveDate::from_ymd(1098, 1, 2), NaiveDate::from_ymd(1099, 9, 21), 3, Court::Unified, None, None));
        m.insert("康和", Era::new("Kouwa (Heian)", "康和", "こうわ（へいあんじだい）", "Kōwa (Heian)", "Kôwa (Heian)", NaiveDate::from_ymd(1099, 9, 21), NaiveDate::from_ymd(1104, 3, 15), 6, Court::Unified, None, None));
        m.insert("長治", Era::new("Chouji", "長治", "ちょうじ", "Chōji", "Tyôzi", NaiveDate::from_ymd(1104, 3, 15), NaiveDate::from_ymd(1106, 5, 20), 3, Court::Unified, None, None));
        m.insert("嘉承", Era::new("Kajou", "嘉承", "かじょう", "Kajō", "Kazyô", NaiveDate::from_ymd(1106, 5, 20), NaiveDate::from_ymd(1108, 9, 16), 3, Court::Unified, None, None));
        m.insert("天仁", Era::new("Ten'nin", "天仁", "てんにん", "Tennin", "Tennin", NaiveDate::from_ymd(1108, 9, 16), NaiveDate::from_ymd(1110, 8, 7), 3, Court::Unified, None, None));
        m.insert("天永", Era::new("Ten'ei", "天永", "てんえい", "Ten'ei", "Ten'ei", NaiveDate::from_ymd(1110, 8, 7), NaiveDate::from_ymd(1113, 9, 1), 4, Court::Unified, None, None));
        m.insert("永久", Era::new("Eikyuu", "永久", "えいきゅう", "Eikyū", "Eikyû", NaiveDate::from_ymd(1113, 9, 1), NaiveDate::from_ymd(1118, 5, 2), 6, Court::Unified, None, None));
        m.insert("元永", Era::new("Gen'ei", "元永", "げんえい", "Gen'ei", "Gen'ei", NaiveDate::from_ymd(1118, 5, 2), NaiveDate::from_ymd(1120, 5, 16), 3, Court::Unified, None, None));
        m.insert("保安", Era::new("Houan", "保安", "ほうあん", "Hōan", "Hôan", NaiveDate::from_ymd(1120, 5, 16), NaiveDate::from_ymd(1124, 5, 25), 5, Court::Unified, None, None));
        m.insert("天治", Era::new("Tenji", "天治", "てんじ", "Tenji", "Tenzi", NaiveDate::from_ymd(1124, 5, 25), NaiveDate::from_ymd(1126, 2, 22), 3, Court::Unified, None, None));	
        m.insert("大治", Era::new("Daiji", "大治", "だいじ", "Daiji", "Daizi", NaiveDate::from_ymd(1126, 2, 22), NaiveDate::from_ymd(1131, 3, 7), 6, Court::Unified, None, None));
        m.insert("天承", Era::new("Tenjou", "天承", "てんじょう", "Tenjō", "Tenzyô", NaiveDate::from_ymd(1131, 3, 7), NaiveDate::from_ymd(1132, 9, 28), 2, Court::Unified, None, None));
        m.insert("長承", Era::new("Choushou", "長承", "ちょうしょう", "Chōshō", "Tyôsyô", NaiveDate::from_ymd(1132, 9, 28), NaiveDate::from_ymd(1135, 6, 17), 4, Court::Unified, None, None));
        m.insert("保延", Era::new("Houen", "保延", "ほうえん", "Hōen", "Hôen", NaiveDate::from_ymd(1135, 6, 17), NaiveDate::from_ymd(1141, 8, 20), 7, Court::Unified, None, None));
        m.insert("永治", Era::new("Eiji", "永治", "えいじ", "Eiji", "Eizi", NaiveDate::from_ymd(1141, 8, 20), NaiveDate::from_ymd(1142, 6, 1), 2, Court::Unified, None, None));
        m.insert("康治", Era::new("Kouji (Heian)", "康治", "こうじ（へいあんじだい）", "Kōji (Heian)", "Kôzi (Heian)", NaiveDate::from_ymd(1142, 6, 1), NaiveDate::from_ymd(1144, 4, 4), 3, Court::Unified, None, None));
        m.insert("天養", Era::new("Ten'you", "天養", "てんよう", "Ten'yō", "Ten'yô", NaiveDate::from_ymd(1144, 4, 4), NaiveDate::from_ymd(1145, 8, 19), 2, Court::Unified, None, None));
        m.insert("久安", Era::new("Kyuuan", "久安", "きゅうあん", "Kyūan", "Kyûan", NaiveDate::from_ymd(1145, 8, 19), NaiveDate::from_ymd(1151, 2, 21), 7, Court::Unified, None, None));
        m.insert("仁平", Era::new("Ninpei", "仁平", "にんぺい", "Ninpei", "Ninpei", NaiveDate::from_ymd(1151, 2, 21), NaiveDate::from_ymd(1154, 12, 21), 4, Court::Unified, None, None));
        m.insert("久寿", Era::new("Kyuuju", "久寿", "きゅうじゅ", "Kyūju", "Kyûzyu", NaiveDate::from_ymd(1154, 12, 21), NaiveDate::from_ymd(1156, 5, 25), 3, Court::Unified, None, None));
        m.insert("保元", Era::new("Hougen", "保元", "ほうげん", "Hōgen", "Hôgen", NaiveDate::from_ymd(1156, 5, 25), NaiveDate::from_ymd(1159, 5, 16), 4, Court::Unified, None, None));
        m.insert("平治", Era::new("Heiji", "平治", "へいじ", "Heiji", "Heizi", NaiveDate::from_ymd(1159, 5, 16), NaiveDate::from_ymd(1160, 2, 25), 2, Court::Unified, None, None));
        m.insert("永暦", Era::new("Eiryaku", "永暦", "えいりゃく", "Eiryaku", "Eiryaku", NaiveDate::from_ymd(1160, 2, 25), NaiveDate::from_ymd(1161, 10, 1), 2, Court::Unified, None, None));
        m.insert("応保", Era::new("Ouhou", "応保", "おうほう", "Ōhō", "Ôhô", NaiveDate::from_ymd(1161, 10, 1), NaiveDate::from_ymd(1163, 5, 11), 3, Court::Unified, None, None));
        m.insert("長寛", Era::new("Choukan", "長寛", "ちょうかん", "Chōkan", "Tyôkan", NaiveDate::from_ymd(1163, 5, 11), NaiveDate::from_ymd(1165, 6, 21), 3, Court::Unified, None, None));
        m.insert("永万", Era::new("Eiman", "永万", "えいまん", "Eiman", "Eiman", NaiveDate::from_ymd(1165, 6, 21), NaiveDate::from_ymd(1166, 9, 30), 2, Court::Unified, None, None));
        m.insert("仁安", Era::new("Nin'an", "仁安", "にんあん", "Nin'an", "Nin'an", NaiveDate::from_ymd(1166, 9, 30), NaiveDate::from_ymd(1169, 5, 13), 4, Court::Unified, None, None));
        m.insert("嘉応", Era::new("Kaou", "嘉応", "かおう", "Kaō", "Kaô", NaiveDate::from_ymd(1169, 5, 13), NaiveDate::from_ymd(1171, 5, 19), 3, Court::Unified, None, None));
        m.insert("承安", Era::new("Jouan", "承安", "じょうあん", "Jōan", "Zyôan", NaiveDate::from_ymd(1171, 5, 19), NaiveDate::from_ymd(1175, 8, 23), 5, Court::Unified, None, None));
        m.insert("安元", Era::new("Angen", "安元", "あんげん", "Angen", "Angen", NaiveDate::from_ymd(1175, 8, 23), NaiveDate::from_ymd(1177, 9, 5), 3, Court::Unified, None, None));
        m.insert("治承", Era::new("Jishou", "治承", "じしょう", "Jishō", "Zisyô", NaiveDate::from_ymd(1177, 9, 5), NaiveDate::from_ymd(1181, 9, 1), 5, Court::Unified, None, None));
        m.insert("養和", Era::new("Youwa", "養和", "ようわ", "Yōwa", "Yôwa", NaiveDate::from_ymd(1181, 9, 1), NaiveDate::from_ymd(1182, 7, 6), 2, Court::Unified, None, None));
        m.insert("寿永", Era::new("Juei", "寿永", "じゅえい", "Juei", "Juei", NaiveDate::from_ymd(1182, 7, 6), NaiveDate::from_ymd(1184, 6, 3), 3, Court::Unified, None, None));
        m.insert("元暦", Era::new("Genryaku", "元暦", "げんりゃく", "Genryaku", "Genryaku", NaiveDate::from_ymd(1184, 6, 3), NaiveDate::from_ymd(1185,9 , 16), 2, Court::Unified, None, None));
        //Kamakura period
        m.insert("文治", Era::new("Bunji", "文治", "ぶんじ", "Bunji", "Bunzi", NaiveDate::from_ymd(1185, 9, 16), NaiveDate::from_ymd(1190, 5, 23), 6, Court::Unified, None, None));
        m.insert("建久", Era::new("Kenkyuu", "建久", "けんきゅう", "Kenkyū", "Kenkyû", NaiveDate::from_ymd(1190, 5, 23), NaiveDate::from_ymd(1199, 5, 30), 10, Court::Unified, None, None));
        m.insert("正治", Era::new("Shouji", "正治", "しょうじ", "Shōji", "Syôzi", NaiveDate::from_ymd(1199, 5, 30), NaiveDate::from_ymd(1201, 3, 26), 3, Court::Unified, None, None));
        m.insert("建仁", Era::new("Ken'nin", "建仁", "けんにん", "Kennin", "Kennin", NaiveDate::from_ymd(1201, 3, 26), NaiveDate::from_ymd(1204, 3, 30), 4, Court::Unified, None, None));
        m.insert("元久", Era::new("Genkyuu", "元久", "げんきゅう", "Genkyū", "Genkyû", NaiveDate::from_ymd(1204, 3, 30), NaiveDate::from_ymd(1206, 6, 12), 3, Court::Unified, None, None));
        m.insert("建永", Era::new("Ken'ei", "建永", "けんえい", "Ken'ei", "Ken'ei", NaiveDate::from_ymd(1206, 6, 12), NaiveDate::from_ymd(1207, 11, 23), 2, Court::Unified, None, None));
        m.insert("承元", Era::new("Jougen (Kamakura)", "承元", "じょうげん（かまくらじだい）", "Jōgen (Kamakura)", "Zyôgen (Kamakura)", NaiveDate::from_ymd(1207, 11, 23), NaiveDate::from_ymd(1211, 4, 30), 5, Court::Unified, None, None));
        m.insert("建暦", Era::new("Kenryaku", "建暦", "けんりゃく", "Kenryaku", "Kenryaku", NaiveDate::from_ymd(1211, 4, 30), NaiveDate::from_ymd(1214, 1, 25), 3, Court::Unified, None, None));
        m.insert("建保", Era::new("Kenpou", "建保", "けんぽう", "Kenpō", "Kenpô", NaiveDate::from_ymd(1214, 1, 25), NaiveDate::from_ymd(1219, 6, 3), 7, Court::Unified, None, None));
        m.insert("承久", Era::new("Joukyuu", "承久", "じょうきゅう", "Jōkyū", "Zyôkyû", NaiveDate::from_ymd(1219, 6, 3), NaiveDate::from_ymd(1222, 6, 1), 4, Court::Unified, None, None));
        m.insert("貞応", Era::new("Jouou (Kamakura)", "貞応", "じょうおう（かまくらじだい）", "Jōō (Kamakura)", "Zyôô (Kamakura)", NaiveDate::from_ymd(1222, 6, 1), NaiveDate::from_ymd(1225, 1, 7), 3, Court::Unified, None, None));
        m.insert("元仁", Era::new("Gen'nin", "元仁", "げんにん", "Gennin", "Gennin", NaiveDate::from_ymd(1225, 1, 7), NaiveDate::from_ymd(1125, 6, 4), 2, Court::Unified, None, None));
        m.insert("嘉禄", Era::new("Karoku", "嘉禄", "かろく", "Karoku", "Karoku", NaiveDate::from_ymd(1125, 6, 4), NaiveDate::from_ymd(1228, 1, 25), 3, Court::Unified, None, None));
        m.insert("安貞", Era::new("Antei", "安貞", "あんてい", "Antei", "Antei", NaiveDate::from_ymd(1228, 1, 25), NaiveDate::from_ymd(1229, 4, 7), 3, Court::Unified, None, None));
        m.insert("寛喜", Era::new("Kanki", "寛喜", "かんき", "Kanki", "Kanki", NaiveDate::from_ymd(1229, 4, 7), NaiveDate::from_ymd(1232, 4, 30), 4, Court::Unified, None, None));
        m.insert("貞永", Era::new("Jouei", "貞永", "じょうえい", "Jōei", "Zyôei", NaiveDate::from_ymd(1232, 4, 30), NaiveDate::from_ymd(1233, 6, 1), 2, Court::Unified, None, None));
        m.insert("天福", Era::new("Tenpuku", "天福", "てんぷく", "Tenpuku", "Tenpuku", NaiveDate::from_ymd(1233, 6, 1), NaiveDate::from_ymd(1234, 12, 4), 2, Court::Unified, None, None));
        m.insert("文暦", Era::new("Bunryaku", "文暦", "ぶんりゃく", "Bunryaku", "Bunryaku", NaiveDate::from_ymd(1234, 12, 4), NaiveDate::from_ymd(1235, 11, 8), 2, Court::Unified, None, None));
        m.insert("嘉禎", Era::new("Katei", "嘉禎", "かてい", "Katei", "Katei", NaiveDate::from_ymd(1235, 11, 8), NaiveDate::from_ymd(1239, 1, 6), 4, Court::Unified, None, None));
        m.insert("暦仁", Era::new("Ryakunin", "暦仁", "りゃくにん", "Ryakunin", "Ryakunin", NaiveDate::from_ymd(1239, 1, 6), NaiveDate::from_ymd(1239, 3, 20), 2, Court::Unified, None, None));
        m.insert("延応", Era::new("En'ou", "延応", "えんおう", "En'ō", "En'ô", NaiveDate::from_ymd(1239, 3, 20), NaiveDate::from_ymd(1240, 8, 12), 2, Court::Unified, None, None));
        m.insert("仁治", Era::new("Ninji", "仁治", "にんじ", "Ninji", "Ninzi", NaiveDate::from_ymd(1240, 8, 12), NaiveDate::from_ymd(1243, 3, 25), 4, Court::Unified, None, None));
        m.insert("寛元", Era::new("Kangen", "寛元", "かんげん", "Kangen", "Kangen", NaiveDate::from_ymd(1243, 3, 25), NaiveDate::from_ymd(1247, 4, 12), 5, Court::Unified, None, None));
        m.insert("宝治", Era::new("Houji", "宝治", "ほうじ", "Hōji", "Hôzi", NaiveDate::from_ymd(1247, 4, 12), NaiveDate::from_ymd(1249, 5, 9), 3, Court::Unified, None, None));
        m.insert("建長", Era::new("Kenchou", "建長", "けんちょう", "Kenchō", "Kentyô", NaiveDate::from_ymd(1249, 5, 9), NaiveDate::from_ymd(1256, 10, 31), 8, Court::Unified, None, None));
        m.insert("康元", Era::new("Kougen", "康元", "こうげん", "Kōgen", "Kôgen", NaiveDate::from_ymd(1256, 10, 31), NaiveDate::from_ymd(1257, 4, 7), 2, Court::Unified, None, None));
        m.insert("正嘉", Era::new("Shouka", "正嘉", "しょうか", "Shōka", "Syôka", NaiveDate::from_ymd(1257, 4, 7), NaiveDate::from_ymd(1259, 4, 27), 3, Court::Unified, None, None));
        m.insert("正元", Era::new("Shougen", "正元", "しょうげん", "Shōgen", "Syôgen", NaiveDate::from_ymd(1259, 4, 27), NaiveDate::from_ymd(1260, 5, 31), 2, Court::Unified, None, None));
        m.insert("文応", Era::new("Bun'ou", "文応", "ぶんおう", "Bun'ō", "Bun'ô", NaiveDate::from_ymd(1260, 5, 31), NaiveDate::from_ymd(1261, 3, 29), 2, Court::Unified, None, None));
        m.insert("弘長", Era::new("Kouchou", "弘長", "こうちょう", "Kōchō", "Kôtyô", NaiveDate::from_ymd(1261, 3, 29), NaiveDate::from_ymd(1264, 4, 3), 4, Court::Unified, None, None));
        m.insert("文永", Era::new("Bun'ei", "文永", "ぶんえい", "Bun'ei", "Bun'ei", NaiveDate::from_ymd(1264, 4, 3), NaiveDate::from_ymd(1275, 5, 29), 12, Court::Unified, None, None));
        m.insert("建治", Era::new("Kenji", "建治", "けんじ", "Kenji", "Kenzi", NaiveDate::from_ymd(1275, 5, 29), NaiveDate::from_ymd(1278, 3, 30), 4, Court::Unified, None, None));
        m.insert("弘安", Era::new("Kouan (Kamakura)", "弘安", "こうあん（かまくらじだい）", "Kōan (Kamakura)", "Kôan (Kamakura)", NaiveDate::from_ymd(1278, 3, 30), NaiveDate::from_ymd(1288, 6, 5), 11, Court::Unified, None, None));
        m.insert("正応", Era::new("Shouou", "正応", "しょうおう", "Shōō", "Syôô", NaiveDate::from_ymd(1288, 6, 5), NaiveDate::from_ymd(1293, 9, 13), 6, Court::Unified, None, None));
        m.insert("永仁", Era::new("Einin", "永仁", "えいにん", "Einin", "Einin", NaiveDate::from_ymd(1293, 9, 13), NaiveDate::from_ymd(1299, 6, 5), 7, Court::Unified, None, None));
        m.insert("正安", Era::new("Shouan", "正安", "しょうあん", "Shōan", "Syôan", NaiveDate::from_ymd(1299, 6, 5), NaiveDate::from_ymd(1302, 12, 18), 4, Court::Unified, None, None));
        m.insert("乾元", Era::new("Kengen", "乾元", "けんげん", "Kengen", "Kengen", NaiveDate::from_ymd(1302, 12, 18), NaiveDate::from_ymd(1303, 9, 24), 2, Court::Unified, None, None));
        m.insert("嘉元", Era::new("Kagen", "嘉元", "かげん", "Kagen", "Kagen", NaiveDate::from_ymd(1303, 9, 24), NaiveDate::from_ymd(1307, 1, 26), 4, Court::Unified, None, None));
        m.insert("徳治", Era::new("Tokuji", "徳治", "とくじ", "Tokuji", "Tokuzi", NaiveDate::from_ymd(1307, 1, 26), NaiveDate::from_ymd(1308, 11, 30), 3, Court::Unified, None, None));
        m.insert("延慶", Era::new("Engyou", "延慶", "えんぎょう", "Engyō", "Engyô", NaiveDate::from_ymd(1308, 11, 30), NaiveDate::from_ymd(1311, 5, 25), 4, Court::Unified, None, None));
        m.insert("応長", Era::new("Ouchou", "応長", "おうちょう", "Ōchō", "Ôtyô", NaiveDate::from_ymd(1311, 5, 25), NaiveDate::from_ymd(1312, 5, 5), 2, Court::Unified, None, None));
        m.insert("正和", Era::new("Shouwa (Kamakura)", "正和", "しょうわ（かまくらじだい）", "Shōwa (Kamakura)", "Syôwa (Kamakura)", NaiveDate::from_ymd(1312, 5, 5), NaiveDate::from_ymd(1317, 3, 24), 6, Court::Unified, None, None));
        m.insert("文保", Era::new("Bunpou", "文保", "ぶんぽう", "Bunpō", "Bunpô", NaiveDate::from_ymd(1317, 3, 24), NaiveDate::from_ymd(1319, 5, 26), 3, Court::Unified, None, None));
        m.insert("元応", Era::new("Gen'ou", "元応", "げんおう", "Gen'ō", "Gen'ô", NaiveDate::from_ymd(1319, 5, 26), NaiveDate::from_ymd(1321, 3, 30), 3, Court::Unified, None, None));
        m.insert("元亨", Era::new("Genkyou", "元亨", "げんきょう", "Genkyō", "Genkyô", NaiveDate::from_ymd(1321, 3, 30), NaiveDate::from_ymd(1325, 1, 2), 4, Court::Unified, None, None));
        m.insert("正中", Era::new("Shouchuu", "正中", "しょうちゅう", "Shōchū", "Syôtyû", NaiveDate::from_ymd(1325, 1, 2), NaiveDate::from_ymd(1326, 6, 5), 3, Court::Unified, None, None));
        m.insert("嘉暦", Era::new("Karyaku", "嘉暦", "かりゃく", "Karyaku", "Karyaku", NaiveDate::from_ymd(1326, 6, 5), NaiveDate::from_ymd(1329, 9, 30), 4, Court::Unified, None, None));
        m.insert("元徳", Era::new("Gentoku", "元徳", "げんとく", "Gentoku", "Gentoku", NaiveDate::from_ymd(1329, 9, 30), NaiveDate::from_ymd(1332, 5, 31), 4, Court::Both, Some(NaiveDate::from_ymd(1331, 9, 19)), Some(3)));
        m.insert("元弘", Era::new("Genkou", "元弘", "げんこう", "Genkō", "Genkô", NaiveDate::from_ymd(1331, 9, 19), NaiveDate::from_ymd(1334, 3, 13), 4, Court::South, None, None));
        m.insert("正慶", Era::new("Shoukyou", "正慶", "しょうきょう", "Shōkyō", "Syôkyô", NaiveDate::from_ymd(1332, 5, 23), NaiveDate::from_ymd(1333, 7, 15), 2, Court::North, None, None));
        //Kenmu restoration
        m.insert("建武", Era::new("Kenmu", "建武", "けんむ", "Kenmu", "Kenmu", NaiveDate::from_ymd(1334, 3, 13), NaiveDate::from_ymd(1338, 10, 19), 5, Court::Both, Some(NaiveDate::from_ymd(1336, 4, 19)), Some(3)));
        //Muromachi/Ashikaga period
        //Northern and Southern Courts period
        m.insert("延元", Era::new("Engen", "延元", "えんげん", "Engen", "Engen", NaiveDate::from_ymd(1336, 4, 19), NaiveDate::from_ymd(1340, 6, 2), 5, Court::South, None, None));
        m.insert("暦応", Era::new("Ryakuou", "暦応", "りゃくおう", "Ryakuō", "Ryakuô", NaiveDate::from_ymd(1338, 10, 19), NaiveDate::from_ymd(1342, 6, 9), 5, Court::North, None, None));
        m.insert("興国", Era::new("Koukoku", "興国", "こうこく", "Kōkoku", "Kôkoku", NaiveDate::from_ymd(1340, 6, 2), NaiveDate::from_ymd(1347, 1, 28), 7, Court::South, None, None));
        m.insert("康永", Era::new("Kouei", "康永", "こうえい", "Kōei", "Kôei", NaiveDate::from_ymd(1342, 6, 9), NaiveDate::from_ymd(1345, 11, 23), 4, Court::North, None, None));
        m.insert("貞和", Era::new("Teiwa", "貞和", "ていわ", "Teiwa", "Teiwa", NaiveDate::from_ymd(1345, 11, 23), NaiveDate::from_ymd(1350, 4, 12), 6, Court::North, None, None));
        m.insert("正平", Era::new("Shouhei", "正平", "しょうへい", "Shōhei", "Syôhei", NaiveDate::from_ymd(1347, 1, 28), NaiveDate::from_ymd(1370, 8, 24), 25, Court::South, None, None));
        m.insert("観応", Era::new("Kan'nou", "観応", "かんのう", "Kannō", "Kannô", NaiveDate::from_ymd(1350, 4, 12), NaiveDate::from_ymd(1352, 11, 12), 3, Court::North, None, None));
        m.insert("文和", Era::new("Bun'na", "文和", "ぶんな", "Bunna", "Bunna", NaiveDate::from_ymd(1352, 11, 12), NaiveDate::from_ymd(1356, 5, 7), 5, Court::North, None, None));
        m.insert("延文", Era::new("Enbun", "延文", "えんぶん", "Enbun", "Enbun", NaiveDate::from_ymd(1356, 5, 7), NaiveDate::from_ymd(1361, 5, 12), 6, Court::North, None, None));
        m.insert("康安", Era::new("Kouan (Muromachi)", "康安", "こうあん（むろまちじだい）", "Kōan (Muromachi)", "Kôan (Muromati)", NaiveDate::from_ymd(1361, 5, 12), NaiveDate::from_ymd(1362, 10, 19), 2, Court::North, None, None));
        m.insert("貞治", Era::new("Jouji", "貞治", "じょうじ", "Jōji", "Zyôzi", NaiveDate::from_ymd(1362, 10, 19), NaiveDate::from_ymd(1368, 3, 15), 7, Court::North, None, None));
        m.insert("応安", Era::new("Ouan", "応安", "おうあん", "Ōan", "Ôan", NaiveDate::from_ymd(1368, 3, 15), NaiveDate::from_ymd(1375, 4, 6), 8, Court::North, None, None));
        m.insert("建徳", Era::new("Kentoku", "建徳", "けんとく", "Kentoku", "Kentoku", NaiveDate::from_ymd(1370, 8, 24), NaiveDate::from_ymd(1372, 6, 8), 3, Court::South, None, None));
        m.insert("文中", Era::new("Bunchuu", "文中", "ぶんちゅう", "Bunchū", "Buntyû", NaiveDate::from_ymd(1372, 5, 9), NaiveDate::from_ymd(1375, 7, 4), 4, Court::South, None, None));
        m.insert("永和", Era::new("Eiwa", "永和", "えいわ", "Eiwa", "Eiwa", NaiveDate::from_ymd(1375, 4, 6), NaiveDate::from_ymd(1379, 4, 17), 5, Court::North, None, None));
        m.insert("天授", Era::new("Tenju", "天授", "てんじゅ", "Tenju", "Tenzyu", NaiveDate::from_ymd(1375, 7, 4), NaiveDate::from_ymd(1381, 3, 14), 7, Court::South, None, None));
        m.insert("康暦", Era::new("Kouryaku", "康暦", "こうりゃく", "Kōryaku", "Kôryaku", NaiveDate::from_ymd(1379, 4, 17), NaiveDate::from_ymd(1381, 3, 28), 3, Court::North, None, None));
        m.insert("弘和", Era::new("Kouwa (Muromachi)", "弘和", "こうわ（むろまちじだい）", "Kōwa (Muromachi)", "Kôwa (Muromati)", NaiveDate::from_ymd(1381, 3, 14), NaiveDate::from_ymd(1384, 5, 26), 4, Court::South, None, None));
        m.insert("永徳", Era::new("Eitoku", "永徳", "えいとく", "Eitoku", "Eitoku", NaiveDate::from_ymd(1381, 3, 28), NaiveDate::from_ymd(1384, 3, 27), 4, Court::North, None, None));
        m.insert("至徳", Era::new("Shitoku", "至徳", "しとく", "Shitoku", "Sitoku", NaiveDate::from_ymd(1384, 3, 27), NaiveDate::from_ymd(1387, 10, 13), 4, Court::North, None, None));
        m.insert("元中", Era::new("Genchuu", "元中", "げんちゅう", "Genchū", "Gentyû", NaiveDate::from_ymd(1384, 5, 26), NaiveDate::from_ymd(1392, 11, 27), 9, Court::South, None, None));
        m.insert("嘉慶", Era::new("Kakei", "嘉慶", "かけい", "Kakei", "Kakei", NaiveDate::from_ymd(1387, 10, 13), NaiveDate::from_ymd(1389, 3, 15), 3, Court::North, None, None));
        m.insert("康応", Era::new("Kouou", "康応", "こうおう", "Kōō", "Kôô", NaiveDate::from_ymd(1389, 3, 15), NaiveDate::from_ymd(1390, 4, 20), 2, Court::North, None, None));
        m.insert("明徳", Era::new("Meitoku", "明徳", "めいとく", "Meitoku", "Meitoku", NaiveDate::from_ymd(1390, 4, 20), NaiveDate::from_ymd(1394, 8, 10), 5, Court::Both, None, None));
        //Courts reunify; Northern and Southern Courts period ends
        //Muromachi/Ashikaga period continues
        m.insert("応永", Era::new("Ouei", "応永", "おうえい", "Ōei", "Ôei", NaiveDate::from_ymd(1394, 8, 10), NaiveDate::from_ymd(1428, 6, 19), 35, Court::Unified, None, None));
        m.insert("正長", Era::new("Shouchou", "正長", "しょうちょう", "Shōchō", "Syôtyô", NaiveDate::from_ymd(1428, 6, 19), NaiveDate::from_ymd(1429, 10, 12), 2, Court::Unified, None, None));
        m.insert("永享", Era::new("Eikyou", "永享", "えいきょう", "Eikyō", "Eikyô", NaiveDate::from_ymd(1429, 10, 12), NaiveDate::from_ymd(1441, 3, 19), 13, Court::Unified, None, None));
        m.insert("嘉吉", Era::new("Kakitsu", "嘉吉", "かきつ", "Kakitsu", "Kakitu", NaiveDate::from_ymd(1441, 3, 19), NaiveDate::from_ymd(1444, 3, 3), 4, Court::Unified, None, None));
        m.insert("文安", Era::new("Bun'an", "文安", "ぶんあん", "Bun'an", "Bun'an", NaiveDate::from_ymd(1444, 3, 3), NaiveDate::from_ymd(1449, 8, 25), 6, Court::Unified, None, None));
        m.insert("宝徳", Era::new("Houtoku", "宝徳", "ほうとく", "Hōtoku", "Hôtoku", NaiveDate::from_ymd(1449, 8, 25), NaiveDate::from_ymd(1452, 8, 19), 4, Court::Unified, None, None));
        m.insert("享徳", Era::new("Kyoutoku", "享徳", "きょうとく", "Kyōtoku", "Kyôtoku", NaiveDate::from_ymd(1452, 8, 19), NaiveDate::from_ymd(1455, 9, 15), 4, Court::Unified, None, None));
        m.insert("康正", Era::new("Koushou", "康正", "こうしょう", "Kōshō", "Kôsyô", NaiveDate::from_ymd(1455, 9, 15), NaiveDate::from_ymd(1457, 10, 25), 3, Court::Unified, None, None));
        m.insert("長禄", Era::new("Chouroku", "長禄", "ちょうろく", "Chōroku", "Tyôroku", NaiveDate::from_ymd(1457, 10, 25), NaiveDate::from_ymd(1461, 2, 10), 4, Court::Unified, None, None));
        m.insert("寛正", Era::new("Kanshou", "寛正", "かんしょう", "Kanshō", "Kansyô", NaiveDate::from_ymd(1461, 2, 10), NaiveDate::from_ymd(1466, 3, 23), 7, Court::Unified, None, None));
        m.insert("文正", Era::new("Bunshou", "文正", "ぶんしょう", "Bunshō", "Bunsyô", NaiveDate::from_ymd(1466, 3, 23), NaiveDate::from_ymd(1467, 4, 18), 2, Court::Unified, None, None));
        //Sengoku/Warring States period
        //Muromachi/Ashikaga period continues
        m.insert("応仁", Era::new("Ounin", "応仁", "おうにん", "Ōnin", "Ônin", NaiveDate::from_ymd(1467, 4, 18), NaiveDate::from_ymd(1469, 6, 17), 3, Court::Unified, None, None));
        m.insert("文明", Era::new("Bunmei", "文明", "ぶんめい", "Bunmei", "Bunmei", NaiveDate::from_ymd(1469, 6, 17), NaiveDate::from_ymd(1487, 8, 18), 19, Court::Unified, None, None));
        m.insert("長享", Era::new("Choukyou", "長享", "ちょうきょう", "Chōkyō", "Tyôkyô", NaiveDate::from_ymd(1487, 8, 18), NaiveDate::from_ymd(1489, 9, 25), 3, Court::Unified, None, None));
        m.insert("延徳", Era::new("Entoku", "延徳", "えんとく", "Entoku", "Entoku", NaiveDate::from_ymd(1489, 9, 25), NaiveDate::from_ymd(1492, 8, 21), 4, Court::Unified, None, None));
        m.insert("明応", Era::new("Meiou", "明応", "めいおう", "Meiō", "Meiô", NaiveDate::from_ymd(1492, 8, 21), NaiveDate::from_ymd(1501, 3, 28), 10, Court::Unified, None, None));
        m.insert("文亀", Era::new("Bunki", "文亀", "ぶんき", "Bunki", "Bunki", NaiveDate::from_ymd(1501, 3, 28), NaiveDate::from_ymd(1504, 3, 26), 4, Court::Unified, None, None));
        m.insert("永正", Era::new("Eishou", "永正", "えいしょう", "Eishō", "Eisyô", NaiveDate::from_ymd(1504, 3, 26), NaiveDate::from_ymd(1521, 10, 3), 18, Court::Unified, None, None));
        m.insert("大永", Era::new("Daiei", "大永", "だいえい", "Daiei", "Daiei", NaiveDate::from_ymd(1521, 10, 3), NaiveDate::from_ymd(1528, 9, 13), 8, Court::Unified, None, None));
        m.insert("享禄", Era::new("Kyouroku", "享禄", "きょうろく", "Kyōroku", "Kyôroku", NaiveDate::from_ymd(1528, 9, 13), NaiveDate::from_ymd(1532, 9, 8), 5, Court::Unified, None, None));
        m.insert("天文", Era::new("Tenbun", "天文", "てんぶん", "Tenbun", "Tenbun", NaiveDate::from_ymd(1532, 9, 8), NaiveDate::from_ymd(1555, 11, 17), 24, Court::Unified, None, None));
        m.insert("弘治", Era::new("Kouji (Muromachi)", "弘治", "こうじ（むろまちじだい）", "Kōji (Muromachi)", "Kôzi (Muromati)", NaiveDate::from_ymd(1555, 11, 17), NaiveDate::from_ymd(1558, 3, 28), 4, Court::Unified, None, None));
        m.insert("永禄", Era::new("Eiroku", "永禄", "えいろく", "Eiroku", "Eiroku", NaiveDate::from_ymd(1558, 3, 28), NaiveDate::from_ymd(1570, 6, 6), 13, Court::Unified, None, None));
        m.insert("元亀", Era::new("Genki", "元亀", "げんき", "Genki", "Genki", NaiveDate::from_ymd(1570, 6, 6), NaiveDate::from_ymd(1573, 9, 4), 4, Court::Unified, None, None));
        //Azuchi-Momoyama period
        m.insert("天正", Era::new("Tenshou", "天正", "てんしょう", "Tenshō", "Tensyô", NaiveDate::from_ymd(1573, 9, 4), NaiveDate::from_ymd(1593, 1, 10), 20, Court::Unified, None, None));
        m.insert("文禄", Era::new("Bunroku", "文禄", "ぶんろく", "Bunroku", "Bunroku", NaiveDate::from_ymd(1593, 1, 10), NaiveDate::from_ymd(1596, 12, 16), 5, Court::Unified, None, None));
        m.insert("慶長", Era::new("Keichou", "慶長", "けいちょう", "Keichō", "Keityô", NaiveDate::from_ymd(1596, 12, 16), NaiveDate::from_ymd(1615, 9, 5), 20, Court::Unified, None, None));
        //Edo/Tokugawa period
        m.insert("元和", Era::new("Gen'na", "元和", "げんな", "Genna", "Genna", NaiveDate::from_ymd(1615, 9, 5), NaiveDate::from_ymd(1624, 4, 17), 10, Court::Unified, None, None));
        m.insert("寛永", Era::new("Kan'ei", "寛永", "かんえい", "Kan'ei", "Kan'ei", NaiveDate::from_ymd(1624, 4, 17), NaiveDate::from_ymd(1645, 1, 13), 21, Court::Unified, None, None));
        m.insert("正保", Era::new("Shouhou", "正保", "しょうほう", "Shōhō", "Syôhô", NaiveDate::from_ymd(1645, 1, 13), NaiveDate::from_ymd(1648, 4, 7), 5, Court::Unified, None, None));
        m.insert("慶安", Era::new("Keian", "慶安", "けいあん", "Keian", "Keian", NaiveDate::from_ymd(1648, 4, 7), NaiveDate::from_ymd(1652, 10, 20), 5, Court::Unified, None, None));
        m.insert("承応", Era::new("Jouou (Edo)", "承応", "じょうおう（えどじだい）", "Jōō (Edo)", "Zyôô (Edo)", NaiveDate::from_ymd(1652, 10, 20), NaiveDate::from_ymd(1655, 5, 18), 4, Court::Unified, None, None));
        m.insert("明暦", Era::new("Meireki", "明暦", "めいれき", "Meireki", "Meireki", NaiveDate::from_ymd(1655, 5, 18), NaiveDate::from_ymd(1658, 8, 21), 4, Court::Unified, None, None));
        m.insert("万治", Era::new("Manji", "万治", "まんじ", "Manji", "Manzi", NaiveDate::from_ymd(1658, 8, 21), NaiveDate::from_ymd(1661, 5, 23), 4, Court::Unified, None, None));
        m.insert("寛文", Era::new("Kanbun", "寛文", "かんぶん", "Kanbun", "Kanbun", NaiveDate::from_ymd(1661, 5, 23), NaiveDate::from_ymd(1673, 10, 30), 13, Court::Unified, None, None));
        m.insert("延宝", Era::new("Enpou", "延宝", "えんぽう", "Enpō", "Enpô", NaiveDate::from_ymd(1673, 10, 30), NaiveDate::from_ymd(1681, 11, 9), 9, Court::Unified, None, None));
        m.insert("天和", Era::new("Ten'na", "天和", "てんな", "Tenna", "Tenna", NaiveDate::from_ymd(1681, 11, 9), NaiveDate::from_ymd(1684, 4, 5), 4, Court::Unified, None, None));
        m.insert("貞享", Era::new("Joukyou", "貞享", "じょうきょう", "Jōkyō", "Zyôkyô", NaiveDate::from_ymd(1684, 4, 5), NaiveDate::from_ymd(1688, 10, 23), 5, Court::Unified, None, None));
        m.insert("元禄", Era::new("Genroku", "元禄", "げんろく", "Genroku", "Genroku", NaiveDate::from_ymd(1688, 10, 23), NaiveDate::from_ymd(1704, 4, 16), 17, Court::Unified, None, None));
        m.insert("宝永", Era::new("Houei", "宝永", "ほうえい", "Hōei", "Hôei", NaiveDate::from_ymd(1704, 4, 16), NaiveDate::from_ymd(1711, 6, 11), 8, Court::Unified, None, None));
        m.insert("正徳", Era::new("Shoutoku", "正徳", "しょうとく", "Shōtoku", "Syôtoku", NaiveDate::from_ymd(1711, 6, 11), NaiveDate::from_ymd(1716, 8, 9), 6, Court::Unified, None, None));
        m.insert("享保", Era::new("Kyouhou", "享保", "きょうほう", "Kyōhō", "Kyôhô", NaiveDate::from_ymd(1716, 8, 9), NaiveDate::from_ymd(1736, 6, 7), 21, Court::Unified, None, None));
        m.insert("元文", Era::new("Genbun", "元文", "げんぶん", "Genbun", "Genbun", NaiveDate::from_ymd(1736, 6, 7), NaiveDate::from_ymd(1741, 4, 12), 6, Court::Unified, None, None));
        m.insert("寛保", Era::new("Kanpou", "寛保", "かんぽう", "Kapō", "Kapô", NaiveDate::from_ymd(1741, 4, 12), NaiveDate::from_ymd(1744, 4, 13), 4, Court::Unified, None, None));
        m.insert("延享", Era::new("Enkyou", "延享", "えんきょう", "Enkyō", "Enkyô", NaiveDate::from_ymd(1744, 4, 13), NaiveDate::from_ymd(1748, 8, 5), 5, Court::Unified, None, None));
        m.insert("寛延", Era::new("Kan'en", "寛延", "かんえん", "Kan'en", "Kan'en", NaiveDate::from_ymd(1748, 8, 5), NaiveDate::from_ymd(1751, 12, 14), 4, Court::Unified, None, None));
        m.insert("宝暦", Era::new("Houreki", "宝暦", "ほうれき", "Hōreki", "Hôreki", NaiveDate::from_ymd(1751, 12, 14), NaiveDate::from_ymd(1764, 6, 30), 14, Court::Unified, None, None));
        m.insert("明和", Era::new("Meiwa", "明和", "めいわ", "Meiwa", "Meiwa", NaiveDate::from_ymd(1764, 6, 30), NaiveDate::from_ymd(1772, 12, 10), 9, Court::Unified, None, None));
        m.insert("安永", Era::new("An'ei", "安永", "あんえい", "An'ei", "An'ei", NaiveDate::from_ymd(1772, 12, 10), NaiveDate::from_ymd(1781, 4, 25), 10, Court::Unified, None, None));
        m.insert("天明", Era::new("Tenmei", "天明", "てんめい", "Tenmei", "Tenmei", NaiveDate::from_ymd(1781, 4, 25), NaiveDate::from_ymd(1789, 2, 19), 9, Court::Unified, None, None));
        m.insert("寛政", Era::new("Kansei", "寛政", "かんせい", "Kansei", "Kansei", NaiveDate::from_ymd(1789, 2, 19), NaiveDate::from_ymd(1801, 3, 19), 13, Court::Unified, None, None));
        m.insert("享和", Era::new("Kyouwa", "享和", "きょうわ", "Kyōwa", "Kyôwa", NaiveDate::from_ymd(1801, 3, 19), NaiveDate::from_ymd(1804, 3, 22), 4, Court::Unified, None, None));
        m.insert("文化", Era::new("Bunka", "文化", "ぶんか", "Bunka", "Bunka", NaiveDate::from_ymd(1804, 3, 22), NaiveDate::from_ymd(1818, 5, 26), 15, Court::Unified, None, None));
        m.insert("文政", Era::new("Bunsei", "文政", "ぶんせい", "Bunsei", "Bunsei", NaiveDate::from_ymd(1818, 5, 26), NaiveDate::from_ymd(1831, 1, 23), 13, Court::Unified, None, None));
        m.insert("天保", Era::new("Tenpou", "天保", "てんぽう", "Tenpō", "Tenpô", NaiveDate::from_ymd(1831, 1, 23), NaiveDate::from_ymd(1845, 1, 9), 15, Court::Unified, None, None));
        m.insert("弘化", Era::new("Kouka", "弘化", "こうか", "Kōka", "Kôka", NaiveDate::from_ymd(1845, 1, 9), NaiveDate::from_ymd(1848, 4, 1), 5, Court::Unified, None, None));
        m.insert("嘉永", Era::new("Kaei", "嘉永", "かえい", "Kaei", "Kaei", NaiveDate::from_ymd(1848, 4, 1), NaiveDate::from_ymd(1855, 1, 15), 7, Court::Unified, None, None));
        m.insert("安政", Era::new("Ansei", "安政", "あんせい", "Ansei", "Ansei", NaiveDate::from_ymd(1855, 1, 15), NaiveDate::from_ymd(1860, 4, 8), 7, Court::Unified, None, None));
        m.insert("万延", Era::new("Man'en", "万延", "まんえん", "Man'en", "Man'en", NaiveDate::from_ymd(1860, 4, 8), NaiveDate::from_ymd(1861, 3, 29), 2, Court::Unified, None, None));
        m.insert("文久", Era::new("Bunkyuu", "文久", "ぶんきゅう", "Bunkyū", "Bunkyû", NaiveDate::from_ymd(1861, 3, 29), NaiveDate::from_ymd(1864, 3, 27), 4, Court::Unified, None, None));
        m.insert("元治", Era::new("Genji", "元治", "げんじ", "Genji", "Genzi", NaiveDate::from_ymd(1864, 3, 27), NaiveDate::from_ymd(1865, 5, 1), 2, Court::Unified, None, None));
        m.insert("慶応", Era::new("Keiou", "慶応", "けいおう", "Keiō", "Keiô", NaiveDate::from_ymd(1865, 5, 1), NaiveDate::from_ymd(1868, 10, 23), 4, Court::Unified, None, None));
        //Modern Japan
        m.insert("明治", Era::new("Meiji", "明治", "めいじ", "Meiji", "Meizi", NaiveDate::from_ymd(1868, 10, 23), NaiveDate::from_ymd(1912, 7, 29), 45, Court::Unified, None, None));
        m.insert("大正", Era::new("Taishou", "大正", "たいしょう", "Taishō", "Taisyô", NaiveDate::from_ymd(1912, 7, 30), NaiveDate::from_ymd(1926, 12, 24), 15, Court::Unified, None, None));
        m.insert("昭和", Era::new("Shouwa", "昭和", "しょうわ", "Shōwa", "Syôwa", NaiveDate::from_ymd(1926, 12, 25), NaiveDate::from_ymd(1989, 1, 7), 64, Court::Unified, None, None));
        m.insert("平成", Era::new("Heisei", "平成", "へいせい", "Heisei", "Heisei", NaiveDate::from_ymd(1989, 1, 8), NaiveDate::from_ymd(2019, 4, 30), 31, Court::Unified, None, None));
        m
    };

    static ref ERA_VEC: Vec<Era> = {
        let mut v = Vec::new();
        v.push(Era::from_kanji("大化").unwrap());
        v.push(Era::from_kanji("白雉").unwrap());
        v.push(Era::from_kanji("朱鳥").unwrap());
        v.push(Era::from_kanji("大宝").unwrap());
        v.push(Era::from_kanji("慶雲").unwrap());
        v.push(Era::from_kanji("和銅").unwrap());
        v.push(Era::from_kanji("霊亀").unwrap());
        v.push(Era::from_kanji("養老").unwrap());
        v.push(Era::from_kanji("神亀").unwrap());
        v.push(Era::from_kanji("天平").unwrap());
        v.push(Era::from_kanji("天平感宝").unwrap());
        v.push(Era::from_kanji("天平勝宝").unwrap());
        v.push(Era::from_kanji("天平宝字").unwrap());
        v.push(Era::from_kanji("天平神護").unwrap());
        v.push(Era::from_kanji("神護景雲").unwrap());
        v.push(Era::from_kanji("宝亀").unwrap());
        v.push(Era::from_kanji("天応").unwrap());
        v.push(Era::from_kanji("延暦").unwrap());
        v.push(Era::from_kanji("大同").unwrap());
        v.push(Era::from_kanji("弘仁").unwrap());
        v.push(Era::from_kanji("天長").unwrap());
        v.push(Era::from_kanji("承和").unwrap());
        v.push(Era::from_kanji("嘉祥").unwrap());
        v.push(Era::from_kanji("仁寿").unwrap());
        v.push(Era::from_kanji("斉衡").unwrap());
        v.push(Era::from_kanji("天安").unwrap());
        v.push(Era::from_kanji("貞観").unwrap());
        v.push(Era::from_kanji("元慶").unwrap());
        v.push(Era::from_kanji("仁和").unwrap());
        v.push(Era::from_kanji("寛平").unwrap());
        v.push(Era::from_kanji("昌泰").unwrap());
        v.push(Era::from_kanji("延喜").unwrap());
        v.push(Era::from_kanji("延長").unwrap());
        v.push(Era::from_kanji("承平").unwrap());
        v.push(Era::from_kanji("天慶").unwrap());
        v.push(Era::from_kanji("天暦").unwrap());
        v.push(Era::from_kanji("天徳").unwrap());
        v.push(Era::from_kanji("応和").unwrap());
        v.push(Era::from_kanji("康保").unwrap());
        v.push(Era::from_kanji("安和").unwrap());
        v.push(Era::from_kanji("天禄").unwrap());
        v.push(Era::from_kanji("天延").unwrap());
        v.push(Era::from_kanji("貞元").unwrap());
        v.push(Era::from_kanji("天元").unwrap()); 
        v.push(Era::from_kanji("永観").unwrap());
        v.push(Era::from_kanji("寛和").unwrap());
        v.push(Era::from_kanji("永延").unwrap());
        v.push(Era::from_kanji("永祚").unwrap());
        v.push(Era::from_kanji("正暦").unwrap());
        v.push(Era::from_kanji("長徳").unwrap());
        v.push(Era::from_kanji("長保").unwrap());
        v.push(Era::from_kanji("寛弘").unwrap());
        v.push(Era::from_kanji("長和").unwrap());
        v.push(Era::from_kanji("寛仁").unwrap());
        v.push(Era::from_kanji("治安").unwrap());
        v.push(Era::from_kanji("万寿").unwrap());
        v.push(Era::from_kanji("長元").unwrap());
        v.push(Era::from_kanji("長暦").unwrap());
        v.push(Era::from_kanji("長久").unwrap());
        v.push(Era::from_kanji("寛徳").unwrap());
        v.push(Era::from_kanji("永承").unwrap());
        v.push(Era::from_kanji("天喜").unwrap());
        v.push(Era::from_kanji("康平").unwrap());
        v.push(Era::from_kanji("治暦").unwrap());
        v.push(Era::from_kanji("延久").unwrap());
        v.push(Era::from_kanji("承保").unwrap());
        v.push(Era::from_kanji("承暦").unwrap());
        v.push(Era::from_kanji("永保").unwrap());
        v.push(Era::from_kanji("応徳").unwrap());
        v.push(Era::from_kanji("寛治").unwrap());
        v.push(Era::from_kanji("嘉保").unwrap());
        v.push(Era::from_kanji("永長").unwrap());
        v.push(Era::from_kanji("承徳").unwrap());
        v.push(Era::from_kanji("康和").unwrap());
        v.push(Era::from_kanji("長治").unwrap());
        v.push(Era::from_kanji("嘉承").unwrap());
        v.push(Era::from_kanji("天仁").unwrap());
        v.push(Era::from_kanji("天永").unwrap());
        v.push(Era::from_kanji("永久").unwrap());
        v.push(Era::from_kanji("元永").unwrap());
        v.push(Era::from_kanji("保安").unwrap());
        v.push(Era::from_kanji("天治").unwrap());
        v.push(Era::from_kanji("大治").unwrap());
        v.push(Era::from_kanji("天承").unwrap());
        v.push(Era::from_kanji("長承").unwrap());
        v.push(Era::from_kanji("保延").unwrap());
        v.push(Era::from_kanji("永治").unwrap());
        v.push(Era::from_kanji("康治").unwrap());
        v.push(Era::from_kanji("天養").unwrap());
        v.push(Era::from_kanji("久安").unwrap());
        v.push(Era::from_kanji("仁平").unwrap());
        v.push(Era::from_kanji("久寿").unwrap());
        v.push(Era::from_kanji("保元").unwrap());
        v.push(Era::from_kanji("平治").unwrap());
        v.push(Era::from_kanji("永暦").unwrap());
        v.push(Era::from_kanji("応保").unwrap());
        v.push(Era::from_kanji("長寛").unwrap());
        v.push(Era::from_kanji("永万").unwrap());
        v.push(Era::from_kanji("仁安").unwrap());
        v.push(Era::from_kanji("嘉応").unwrap());
        v.push(Era::from_kanji("承安").unwrap());
        v.push(Era::from_kanji("安元").unwrap());
        v.push(Era::from_kanji("治承").unwrap());
        v.push(Era::from_kanji("養和").unwrap());
        v.push(Era::from_kanji("寿永").unwrap());
        v.push(Era::from_kanji("元暦").unwrap());
        v.push(Era::from_kanji("文治").unwrap());
        v.push(Era::from_kanji("建久").unwrap());
        v.push(Era::from_kanji("正治").unwrap());
        v.push(Era::from_kanji("建仁").unwrap());
        v.push(Era::from_kanji("元久").unwrap());
        v.push(Era::from_kanji("建永").unwrap());
        v.push(Era::from_kanji("承元").unwrap());
        v.push(Era::from_kanji("建暦").unwrap());
        v.push(Era::from_kanji("建保").unwrap());
        v.push(Era::from_kanji("承久").unwrap());
        v.push(Era::from_kanji("貞応").unwrap());
        v.push(Era::from_kanji("元仁").unwrap());
        v.push(Era::from_kanji("嘉禄").unwrap());
        v.push(Era::from_kanji("安貞").unwrap());
        v.push(Era::from_kanji("寛喜").unwrap());
        v.push(Era::from_kanji("貞永").unwrap());
        v.push(Era::from_kanji("天福").unwrap());
        v.push(Era::from_kanji("文暦").unwrap());
        v.push(Era::from_kanji("嘉禎").unwrap());
        v.push(Era::from_kanji("暦仁").unwrap());
        v.push(Era::from_kanji("延応").unwrap());
        v.push(Era::from_kanji("仁治").unwrap());
        v.push(Era::from_kanji("寛元").unwrap());
        v.push(Era::from_kanji("宝治").unwrap());
        v.push(Era::from_kanji("建長").unwrap());
        v.push(Era::from_kanji("康元").unwrap());
        v.push(Era::from_kanji("正嘉").unwrap());
        v.push(Era::from_kanji("正元").unwrap());
        v.push(Era::from_kanji("文応").unwrap());
        v.push(Era::from_kanji("弘長").unwrap());
        v.push(Era::from_kanji("文永").unwrap());
        v.push(Era::from_kanji("建治").unwrap());
        v.push(Era::from_kanji("弘安").unwrap());
        v.push(Era::from_kanji("正応").unwrap());
        v.push(Era::from_kanji("永仁").unwrap());
        v.push(Era::from_kanji("正安").unwrap());
        v.push(Era::from_kanji("乾元").unwrap());
        v.push(Era::from_kanji("嘉元").unwrap());
        v.push(Era::from_kanji("徳治").unwrap());
        v.push(Era::from_kanji("延慶").unwrap());
        v.push(Era::from_kanji("応長").unwrap());
        v.push(Era::from_kanji("正和").unwrap());
        v.push(Era::from_kanji("文保").unwrap());
        v.push(Era::from_kanji("元応").unwrap());
        v.push(Era::from_kanji("元亨").unwrap());
        v.push(Era::from_kanji("正中").unwrap());
        v.push(Era::from_kanji("嘉暦").unwrap());
        v.push(Era::from_kanji("元徳").unwrap());
        v.push(Era::from_kanji("元弘").unwrap());
        v.push(Era::from_kanji("正慶").unwrap());
        v.push(Era::from_kanji("建武").unwrap());
        v.push(Era::from_kanji("延元").unwrap());
        v.push(Era::from_kanji("暦応").unwrap());
        v.push(Era::from_kanji("興国").unwrap());
        v.push(Era::from_kanji("康永").unwrap());
        v.push(Era::from_kanji("貞和").unwrap());
        v.push(Era::from_kanji("正平").unwrap());
        v.push(Era::from_kanji("観応").unwrap());
        v.push(Era::from_kanji("文和").unwrap());
        v.push(Era::from_kanji("延文").unwrap());
        v.push(Era::from_kanji("康安").unwrap());
        v.push(Era::from_kanji("貞治").unwrap());
        v.push(Era::from_kanji("応安").unwrap());
        v.push(Era::from_kanji("建徳").unwrap());
        v.push(Era::from_kanji("文中").unwrap());
        v.push(Era::from_kanji("永和").unwrap());
        v.push(Era::from_kanji("天授").unwrap());
        v.push(Era::from_kanji("康暦").unwrap());
        v.push(Era::from_kanji("弘和").unwrap());
        v.push(Era::from_kanji("永徳").unwrap());
        v.push(Era::from_kanji("至徳").unwrap());
        v.push(Era::from_kanji("元中").unwrap());
        v.push(Era::from_kanji("嘉慶").unwrap());
        v.push(Era::from_kanji("康応").unwrap());
        v.push(Era::from_kanji("明徳").unwrap());
        v.push(Era::from_kanji("応永").unwrap());
        v.push(Era::from_kanji("正長").unwrap());
        v.push(Era::from_kanji("永享").unwrap());
        v.push(Era::from_kanji("嘉吉").unwrap());
        v.push(Era::from_kanji("文安").unwrap());
        v.push(Era::from_kanji("宝徳").unwrap());
        v.push(Era::from_kanji("享徳").unwrap());
        v.push(Era::from_kanji("康正").unwrap());
        v.push(Era::from_kanji("長禄").unwrap());
        v.push(Era::from_kanji("寛正").unwrap());
        v.push(Era::from_kanji("文正").unwrap());
        v.push(Era::from_kanji("応仁").unwrap());
        v.push(Era::from_kanji("文明").unwrap());
        v.push(Era::from_kanji("長享").unwrap());
        v.push(Era::from_kanji("延徳").unwrap());
        v.push(Era::from_kanji("明応").unwrap());
        v.push(Era::from_kanji("文亀").unwrap());
        v.push(Era::from_kanji("永正").unwrap());
        v.push(Era::from_kanji("大永").unwrap());
        v.push(Era::from_kanji("享禄").unwrap());
        v.push(Era::from_kanji("天文").unwrap());
        v.push(Era::from_kanji("弘治").unwrap());
        v.push(Era::from_kanji("永禄").unwrap());
        v.push(Era::from_kanji("元亀").unwrap());
        v.push(Era::from_kanji("天正").unwrap());
        v.push(Era::from_kanji("文禄").unwrap());
        v.push(Era::from_kanji("慶長").unwrap());
        v.push(Era::from_kanji("元和").unwrap());
        v.push(Era::from_kanji("寛永").unwrap());
        v.push(Era::from_kanji("正保").unwrap());
        v.push(Era::from_kanji("慶安").unwrap());
        v.push(Era::from_kanji("承応").unwrap());
        v.push(Era::from_kanji("明暦").unwrap());
        v.push(Era::from_kanji("万治").unwrap());
        v.push(Era::from_kanji("寛文").unwrap());
        v.push(Era::from_kanji("延宝").unwrap());
        v.push(Era::from_kanji("天和").unwrap());
        v.push(Era::from_kanji("貞享").unwrap());
        v.push(Era::from_kanji("元禄").unwrap());
        v.push(Era::from_kanji("宝永").unwrap());
        v.push(Era::from_kanji("正徳").unwrap());
        v.push(Era::from_kanji("享保").unwrap());
        v.push(Era::from_kanji("元文").unwrap());
        v.push(Era::from_kanji("寛保").unwrap());
        v.push(Era::from_kanji("延享").unwrap());
        v.push(Era::from_kanji("寛延").unwrap());
        v.push(Era::from_kanji("宝暦").unwrap());
        v.push(Era::from_kanji("明和").unwrap());
        v.push(Era::from_kanji("安永").unwrap());
        v.push(Era::from_kanji("天明").unwrap());
        v.push(Era::from_kanji("寛政").unwrap());
        v.push(Era::from_kanji("享和").unwrap());
        v.push(Era::from_kanji("文化").unwrap());
        v.push(Era::from_kanji("文政").unwrap());
        v.push(Era::from_kanji("天保").unwrap());
        v.push(Era::from_kanji("弘化").unwrap());
        v.push(Era::from_kanji("嘉永").unwrap());
        v.push(Era::from_kanji("安政").unwrap());
        v.push(Era::from_kanji("万延").unwrap());
        v.push(Era::from_kanji("文久").unwrap());
        v.push(Era::from_kanji("元治").unwrap());
        v.push(Era::from_kanji("慶応").unwrap());
        v.push(Era::from_kanji("明治").unwrap());
        v.push(Era::from_kanji("大正").unwrap());
        v.push(Era::from_kanji("昭和").unwrap());
        v.push(Era::from_kanji("平成").unwrap());
        v
    };
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Court {
    Unified,
    North,
    South,
    Both,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Era {
    kana_spelling: String,
    kanji: String,
    hiragana: String,
    hepburn: String,
    iso_3602: String,
    start_georgian: NaiveDate,
    end_georgian: NaiveDate,
    end_year: u32,
    court: Court,
    alt_end_georgian: Option<NaiveDate>,
    alt_end_year: Option<u32>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct EraYear {
    era: Era,
    year: u32,
    court: Court,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Date {
    day: u32,
    month: u32,
    year: EraYear,
}

impl Era {
    pub fn new(kana_spelling: &'static str, kanji: &'static str, hiragana: &'static str, hepburn: &'static str, iso_3602: &'static str, start_georgian: NaiveDate, end_georgian: NaiveDate, end_year: u32, court: Court, alt_end_georgian: Option<NaiveDate>, alt_end_year: Option<u32>) -> Self {
        let era = Era {
            kana_spelling: kana_spelling.to_string(),
            kanji: kanji.to_string(),
            hiragana: hiragana.to_string(),
            hepburn: hepburn.to_string(),
            iso_3602: iso_3602.to_string(),
            start_georgian: start_georgian,
            end_georgian: end_georgian,
            end_year: end_year,
            court: court,
            alt_end_georgian: alt_end_georgian,
            alt_end_year: alt_end_year,
        };
        era
    }

    pub fn from_kanji(kanji: &str) -> Result<Self, &'static str> {
        let mut kanji_iter = kanji.chars();
        let length = kanji_iter.clone().count();
        let mut formatted_kanji = "".to_string();
        for _ in 0..length {
            let current = kanji_iter.next().unwrap();
            if is_kanji(current) {
                formatted_kanji.push(current.clone());
            }
        }

        if ERAS.contains_key(formatted_kanji.as_str()) {
            let era = ERAS.get(formatted_kanji.as_str()).unwrap().clone();
            Ok(era)
        }
        else {
            Err("Not a valid era!")
        }
    }

    pub fn from_georgian_year(year: u32) -> Result<Vec<Self>, &'static str> {
        if year < 645 {
            Err("There are no official eras prior to 645")   
        }
        else if year > 654 && year < 686 {
            Err("There was no official era between 656 and 685")
        }
        else if year > 686 && year < 701 {
            Err("There was no official era between 687 and 700")
        }
        else if year > 2019 {
            Err("The era set to begin when the Heisei era ends has not been named yet")
        }
        else {
            let mut eras : Vec<Era> = Vec::new();
            for i in 0..ERA_VEC.len() {
                if ERA_VEC[i].georgian_start_year() <= year {
                    if ERA_VEC[i].georgian_end_year() >= year {
                        eras.push(ERA_VEC[i].clone());
                    }
                }
                else {
                    break;
                }
            }
            Ok(eras)
        }
    }

    pub fn from_georgian(date: NaiveDate) -> Result<Vec<Self>, &'static str> {
        if date.year() < 1 {
            Err("There are no official eras prior to 645")
        }
        else {
            let year = date.year() as u32;

            if year < 645 {
                Err("There are no official eras prior to 645")   
            }
            else if year > 654 && year < 686 {
                Err("There was no official era between 656 and 685")
            }
            else if year > 686 && year < 701 {
                Err("There was no official era between 687 and 700")
            }
            else if year > 2019 {
                Err("The era set to begin when the Heisei era ends has not been named yet")
            }
            else {
                let possible_eras = Era::from_georgian_year(year).unwrap();
                let mut eras : Vec<Era> = Vec::new();

                for i in 0..possible_eras.len() {
                    if date.ge(&possible_eras[i].gsd()) && date.le(&possible_eras[i].ged()) {
                        eras.push(possible_eras[i].clone());
                    }
                }
                Ok(eras)
            }
        }
    }

    pub fn from_gymd(year: i32, month: u32, day: u32) -> Result<Vec<Self>, &'static str> {
        let date = NaiveDate::from_ymd(year, month, day);
        Era::from_georgian(date)
    }

    pub fn georgian_start_year(&self) -> u32 {
        self.start_georgian.year() as u32
    }

    pub fn georgian_end_year(&self) -> u32 {
        self.end_georgian.year() as u32
    }

    pub fn end_year(&self) -> u32 {
        self.end_year
    }

    pub fn kanji(&self) -> String {
        self.kanji.clone()
    }

    //georgian start date
    pub fn gsd(&self) -> NaiveDate {
        self.start_georgian.clone()
    }

    //georgian end date
    pub fn ged(&self) -> NaiveDate {
        self.end_georgian.clone()
    }

    pub fn has_alt_end_date(&self) -> bool {
        self.alt_end_georgian.is_some()
    }

    //alt georgian end date
    //panics if fails
    pub fn gaed(&self) -> NaiveDate {
        if self.has_alt_end_date() {
            self.alt_end_georgian.unwrap().clone()
        }
        else {
            panic!("No alternative end date exists for this era!");
        }
    }

    //alt georgian end date with option
    pub fn gaed_option(&self) -> Option<NaiveDate> {
        self.alt_end_georgian.clone()
    }

    //alt georgian end year
    //panics if fails
    pub fn aey(&self) -> u32 {
        if self.has_alt_end_date() {
            self.alt_end_year.unwrap().clone()
        }
        else {
            panic!("No alternate end year exists for this era!");
        }
    }

    //alt georgian end year with option
    pub fn aey_option(&self) -> Option<u32> {
        self.alt_end_year.clone()
    }

    pub fn court(&self) -> Court {
        self.court.clone()
    }

    pub fn hepburn(&self) -> String {
        self.hepburn.clone()
    }

    pub fn iso_3602(&self) -> String {
        self.iso_3602.clone()
    }

    pub fn kana_spelling(&self) -> String {
        self.kana_spelling.clone()
    }

    pub fn hiragana(&self) -> String {
        self.hiragana.clone()
    }
}

impl EraYear {
    pub fn new(era: Era, year: u32) -> Result<Self, &'static str> {
        if ERAS.contains_key(&era.kanji().as_str()) {
            if Era::from_kanji(&era.kanji()).unwrap() != era {
                Err("Not a valid era!")
            }
            else if year > era.end_year() || year == 0 {
                Err("Not a valid year for this era!")
            }
            else if era.kanji() == "明徳" && year < 4 {
                let era_year = EraYear {
                    era: era,
                    year: year,
                    court: Court::North,
                };
                Ok(era_year)
            }
            else if era.has_alt_end_date() && year > era.aey() {
                let era_year = EraYear {
                    era: era,
                    year: year,
                    court: Court::North,
                };
                Ok(era_year)
            }
            else {
                let era_year = EraYear {
                    era: era.clone(),
                    year: year,
                    court: era.court(),
                };
                Ok(era_year)
            }
        }
        else {
            Err("Not a valid era!")
        }
    }

    pub fn from_georgian(date: NaiveDate) -> Result<Vec<Self>, &'static str> {
        if date.lt(&NaiveDate::from_ymd(645, 7, 20)) {
            Err("There are no official eras prior to July 20th, 645!")
        }
        else if date.gt(&NaiveDate::from_ymd(654, 11, 27)) && date.lt(&NaiveDate::from_ymd(686, 8, 17)) {
            Err("There is no official era between November 28th, 654 and August 16th, 686!")
        }
        else if date.gt(&NaiveDate::from_ymd(686, 10, 4)) && date.lt(&NaiveDate::from_ymd(701, 5, 7)) {
            Err("There is no official era between October 5th, 686 and May 6th, 701!")
        }
        else if date.gt(&NaiveDate::from_ymd(2019, 4, 30)) {
            Err("The era after Heisei has yet to be named!")
        }
        else {
            let dy = date.year() as u32;
            let eras = Era::from_georgian(date).unwrap();
            let mut era_years : Vec<EraYear> = Vec::new();
            for i in 0..eras.len() {
                if date.ge(&eras[i].gsd()) && date.le(&eras[i].ged()) {
                    let mut y = 1;
                    let mut current = eras[i].georgian_start_year();
                    let gsd = eras[i].gsd();
                    let mut gy = 0;

                    if gsd.year() <= 1872 {
                        let year1lny = LUNAR_NEW_YEAR.get(&current).unwrap().clone();

                        if gsd.lt(&year1lny) && date.ge(&year1lny) {
                            y += 1;
                        }
                        current += 1;

                        while current <= dy && current <= 1872 {
                            let lny = LUNAR_NEW_YEAR.get(&current).unwrap().clone();

                            if date.ge(&lny) {
                                y += 1;
                            }
                            current += 1;
                        }
                        //in case the era of the date has dates in both traditional and Georgian calendar formats
                        if current <= dy {
                            gy = dy - current + 1;
                        }
                    }
                    else {
                        gy = dy - eras[i].georgian_start_year();
                    }
                    y += gy;
                    let ey = EraYear::new(eras[i].clone(), y);
                    era_years.push(ey.unwrap());
                }
            }
            Ok(era_years)
        }
    }

    pub fn to_georgian_year(&self) -> Vec<u32> {
        let gsd = self.era.gsd();
        let ged = self.era.ged();
        let end_year = self.era.end_year();
        let mut y = gsd.year() as u32;
        let mut m : HashMap<u32, Vec<u32>> = HashMap::new();

        for i in 0..end_year {
            let mut y1 = y + i;

            if y1 <= 1872 {
                let lny = LUNAR_NEW_YEAR.get(&y).unwrap();

                if y1 == y {
                    m.insert(i + 1, vec![y1]);
                    if gsd.lt(&lny) {
                        y = y - 1;
                    } 
                }
                else {
                    if ged.lt(&lny) {
                        m.insert(i + 1, vec![y1]);
                    }
                    else {
                        m.insert(i + 1, vec![y1, y1 + 1]);
                    }
                }
            }
            else {
                m.insert(i + 1, vec![y1]);
            }
        }

        m.get(&self.year()).unwrap().clone()
    }

    pub fn era(&self) -> Era {
        self.era.clone()
    }

    pub fn year(&self) -> u32 {
        self.year
    }
}

impl Date {
    pub fn new(year: EraYear, month: u32, day: u32) -> Self {
        let date = Date {
            year: year,
            month: month,
            day: day,
        };
        date
    }

    pub fn from_georgian(gdate: NaiveDate) -> Result<Vec<Self>, &'static str> {
        if gdate.lt(&NaiveDate::from_ymd(645, 7, 20)) {
            Err("There are no official eras prior to July 20th, 645!")
        }
        else if gdate.gt(&NaiveDate::from_ymd(654, 11, 27)) && gdate.lt(&NaiveDate::from_ymd(686, 8, 17)) {
            Err("There is not an official era between November 28th, 654 and August 16th, 686!")
        }
        else if gdate.gt(&NaiveDate::from_ymd(686, 10, 4)) && gdate.lt(&NaiveDate::from_ymd(701, 5, 7)) {
            Err("There is not an official era between October 5th, 686 and May 6th, 701!")
        }
        else if gdate.gt(&NaiveDate::from_ymd(2019, 4, 30)) {
            Err("The era after Heisei has yet to be named!")
        }
        else {
            let era_years : Vec<EraYear> = EraYear::from_georgian(gdate).unwrap();
            let mut era_dates : Vec<Date> = Vec::new();

            for i in 0..era_years.len() {
                let era_date = Date::new(era_years[i].clone(), gdate.month(), gdate.day());
                era_dates.push(era_date);
            }
            Ok(era_dates)
        }
    }

    pub fn to_georgian(&self) -> NaiveDate {
        let era = self.era_year().era().clone();
        let end_yn = self.era_year().year();
        let gsd = era.gsd();
        let mut y = gsd.year() as u32;
        let mut yn = 1;

        if y <= 1872 {
            let lny1 = LUNAR_NEW_YEAR.get(&y).unwrap();
            if gsd.lt(&lny1) {
                yn += 1;
            }

            for _ in yn..end_yn {
                y += 1;
            }

            if y <= 1872 {
                let test_date = NaiveDate::from_ymd(y as i32, self.month, self.day);
                let lny = LUNAR_NEW_YEAR.get(&y).unwrap();
                if test_date.lt(&lny) {
                    y += 1;
                }
            }
        }
        else {
            y += end_yn;
            y = y - 1;
        }

        NaiveDate::from_ymd(y as i32, self.month, self.day)
    }

    pub fn month(&self) -> u32 {
        self.month
    }

    pub fn day(&self) -> u32 {
        self.day
    }

    pub fn era_year(&self) -> EraYear {
        self.year.clone()
    }
}
