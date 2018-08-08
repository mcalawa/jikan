// This is part of Jikan
// See README.md for details

use chrono::prelude::*;
use jpn::period::Period;
use std::collections::HashMap;

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

    static ref ERAS_FROM_KANJI: HashMap<&'static str, Era> = {
        let mut m = HashMap::new();
        m.insert("大化", Era::new("Taika", "大化", "たいか", "Taika", "Taika", NaiveDate::from_ymd(645, 7, 20), NaiveDate::from_ymd(650, 3, 25), 6, Court::Unified, None, None));
        m.insert("白雉", Era::new("Hakuchi", "白雉", "はくち", "Hakuchi", "Hakuti", NaiveDate::from_ymd(650, 3, 25), NaiveDate::from_ymd(654, 11, 27), 5, Court::Unified, None, None));
        m.insert("朱鳥", Era::new("Shuchou", "朱鳥", "しゅちょう", "Shuchō", "Syutyô", NaiveDate::from_ymd(686, 8, 17), NaiveDate::from_ymd(686, 10, 4), 1, Court::Unified, None, None));
        m.insert("大宝", Era::new("Taihou", "大宝", "たいほう", "Taihō", "Taihô", NaiveDate::from_ymd(701, 5, 7), NaiveDate::from_ymd(704, 6, 20), 4, Court::Unified, None, None));
        m.insert("慶雲", Era::new("Keiun", "慶雲", "けいうん", "Keiun", "Keiun", NaiveDate::from_ymd(704, 6, 20), NaiveDate::from_ymd(708, 2, 11), 5, Court::Unified, None, None));
        m.insert("和銅", Era::new("Wadou", "和銅", "わどう", "Wadō", "Wadô", NaiveDate::from_ymd(708, 2, 11), NaiveDate::from_ymd(715, 10, 7), 8, Court::Unified, None, None));
        m.insert("霊亀", Era::new("Reiki", "霊亀", "れいき", "Reiki", "Reiki", NaiveDate::from_ymd(715, 10, 7), NaiveDate::from_ymd(717, 12, 28), 3, Court::Unified, None, None));
        //not yet added to ERAS
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
        m.insert("安和", Era::new("Anna", "安和", "あんな", "Anna", "Anna", NaiveDate::from_ymd(968, 9, 13), NaiveDate::from_ymd(970, 5, 8), 3, Court::Unified, None, None));
        m.insert("天禄", Era::new("Tenroku", "天禄", "てんろく", "Tenroku", "Tenroku", NaiveDate::from_ymd(970, 5, 8), NaiveDate::from_ymd(974, 1, 21), 4, Court::Unified, None, None));
        m.insert("天延", Era::new("Ten'en", "天延", "てんえん", "Ten'en", "Ten'en", NaiveDate::from_ymd(974, 1, 21), NaiveDate::from_ymd(976, 8, 16), 4, Court::Unified, None, None));	
        m.insert("貞元", Era::new("Jougen (Heian)", "貞元", "じょうげん（へいあんじだい）", "Jōgen (Heian)", "Zyôgen (Heian)", NaiveDate::from_ymd(976, 8, 16), NaiveDate::from_ymd(979, 1, 5), 3, Court::Unified, None, None));
        m.insert("天元", Era::new("Tengen", "天元", "てんげん", "Tengen", "Tengen", NaiveDate::from_ymd(979, 1, 5), NaiveDate::from_ymd(983, 6, 3), 6, Court::Unified, None, None));
        m.insert("永観", Era::new("Eikan", "永観", "えいかん", "Eikan", "Eikan", NaiveDate::from_ymd(983, 6, 3), NaiveDate::from_ymd(985, 5, 24), 3, Court::Unified, None, None));
        m.insert("寛和", Era::new("Kanna", "寛和", "かんな", "Kanna", "Kanna", NaiveDate::from_ymd(985, 5, 24), NaiveDate::from_ymd(987, 5, 10), 3, Court::Unified, None, None));
        m.insert("永延", Era::new("Eien", "永延", "えいえん", "Eien", "Eien", NaiveDate::from_ymd(987, 5, 10), NaiveDate::from_ymd(989, 9, 15), 3, Court::Unified, None, None));	
        m.insert("永祚", Era::new("Eiso", "永祚", "えいそ", "Eiso", "Eiso", NaiveDate::from_ymd(989, 9, 15), NaiveDate::from_ymd(990, 12, 1), 2, Court::Unified, None, None));
        m.insert("正暦", Era::new("Shouryaku", "正暦", "しょうりゃく", "Shōryaku", "Syôryaku", NaiveDate::from_ymd(990, 12, 1), NaiveDate::from_ymd(995, 3, 30), 6, Court::Unified, None, None));
        m.insert("長徳", Era::new("Choutoku", "長徳", "ちょうとく", "Chōtoku", "Tyôtoku", NaiveDate::from_ymd(995, 3, 30), NaiveDate::from_ymd(999, 2, 6), 5, Court::Unified, None, None));
        m.insert("長保", Era::new("Chouhou", "長保", "ちょうほう", "Chōhō", "Tyôhô", NaiveDate::from_ymd(999, 2, 6), NaiveDate::from_ymd(1004, 8, 14), 6, Court::Unified, None, None));
        m.insert("寛弘", Era::new("Kankou", "寛弘", "かんこう", "Kankō", "Kankô", NaiveDate::from_ymd(1004, 8, 14), NaiveDate::from_ymd(1013, 2, 4), 9, Court::Unified, None, None));
        m.insert("長和", Era::new("Chouwa", "長和", "ちょうわ", "Chōwa", "Tyôwa", NaiveDate::from_ymd(1013, 2, 4), NaiveDate::from_ymd(, , ), 6, Court::Unified, None, None));
        m.insert("寛仁", Era::new("Kannin", "寛仁", "かんにん", "Kannin", "Kannin", NaiveDate::from_ymd(1017, 5, 27), NaiveDate::from_ymd(1021, 3, 23), 5, Court::Unified, None, None));
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
        m.insert("天仁", Era::new("Tennin", "天仁", "てんにん", "Tennin", "Tennin", NaiveDate::from_ymd(1108, 9, 16), NaiveDate::from_ymd(1110, 8, 7), 3, Court::Unified, None, None));
        m.insert("天永", Era::new("Ten'ei", "天永", "てんえい", "Ten'ei", "Ten'ei", NaiveDate::from_ymd(1110, 8, 7), NaiveDate::from_ymd(1113, 9, 1), 4, Court::Unified, None, None));
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(, , ), NaiveDate::from_ymd(, , ), , Court::Unified, None, None));
// Eikyuu	永久	えいきゅう	Eikyū	Eikyû	9/1/1113	5/2/1118	6
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(, , ), NaiveDate::from_ymd(, , ), , Court::Unified, None, None));
// Gen'ei	元永	げんえい	Gen'ei	Gen'ei	5/2/1118	5/16/1120	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(, , ), NaiveDate::from_ymd(, , ), , Court::Unified, None, None));
// Houan	保安	ほうあん	Hōan	Hôan	5/16/1120	5/25/1124	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(, , ), NaiveDate::from_ymd(, , ), , Court::Unified, None, None));
// Tenji	天治	てんじ	Tenji	Tenzi	5/25/1124	2/22/1126	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(, , ), NaiveDate::from_ymd(, , ), , Court::Unified, None, None));
// Daiji	大治	だいじ	Daiji	Daizi	2/22/1126	3/7/1131	6
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Tenjou	天承	てんじょう	Tenjō	Tenzyô	3/7/1131	9/28/1132	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Choushou	長承	ちょうしょう	Chōshō	Tyôsyô	9/28/1132	6/17/1135	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Houen	保延	ほうえん	Hōen	Hôen	6/17/1135	8/20/1141	7
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Eiji	永治	えいじ	Eiji	Eizi	8/20/1141	6/1/1142	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kouji	康治	こうじ	Kōji	Kôzi	6/1/1142	4/4/1144	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Ten'you	天養	てんよう	Ten'yō	Ten'yô	4/4/1144	8/19/1145	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kyuuan	久安	きゅうあん	Kyūan	Kyûan	8/19/1145	2/21/1151	7
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Ninpei	仁平	にんぺい	Ninpei	Ninpei	2/21/1151	12/21/1154	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kyuuju	久寿	きゅうじゅ	Kyūju	Kyûzyu	12/21/1154	5/25/1156	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Hougen	保元	ほうげん	Hōgen	Hôgen	5/25/1156	5/16/1159	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Heiji	平治	へいじ	Heiji	Heizi	5/16/1159	2/25/1160	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Eiryaku	永暦	えいりゃく	Eiryaku	Eiryaku	2/25/1160	10/1/1161	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Ouhou	応保	おうほう	Ōhō	Ôhô	10/1/1161	5/11/1163	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Choukan	長寛	ちょうかん	Chōkan	Tyôkan	5/11/1163	6/21/1165	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Eiman	永万	えいまん	Eiman	Eiman	6/21/1165	9/30/1166	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Nin'an	仁安	にんあん	Nin'an	Nin'an	9/30/1166	5/13/1169	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kaou	嘉応	かおう	Kaō	Kaô	5/13/1169	5/19/1171	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Jouan	承安	じょうあん	Jōan	Zyôan	5/19/1171	8/23/1175	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Angen	安元	あんげん	Angen	Angen	8/23/1175	9/5/1177	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Jishou	治承	じしょう	Jishō	Zisyô	9/5/1177	9/1/1181	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Youwa	養和	ようわ	Yōwa	Yôwa	9/1/1181	7/6/1182	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Juei	寿永	じゅえい	Juei	Juei	7/6/1182	6/3/1184	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Genryaku	元暦	げんりゃく	Genryaku	Genryaku	6/3/1184	9/16/1185	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Bunji	文治	ぶんじ	Bunji	Bunzi	9/16/1185	5/23/1190	6
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kenkyuu	建久	けんきゅう	Kenkyū	Kenkyû	5/23/1190	5/30/1199	10
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Shouji	正治	しょうじ	Shōji	Syôzi	5/30/1199	3/26/1201	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kennin	建仁	けんにん	Kennin	Kennin	3/26/1201	3/30/1204	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Genkyuu	元久	げんきゅう	Genkyū	Genkyû	3/30/1204	6/12/1206	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Ken'ei	建永	けんえい	Ken'ei	Ken'ei	6/12/1206	11/23/1207	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Jougen	承元	じょうげん	Jōgen	Zyôgen	11/23/1207	4/30/1211	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kenryaku	建暦	けんりゃく	Kenryaku	Kenryaku	4/30/1211	1/25/1214	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kenpou	建保	けんぽう	Kenpō	Kenpô	1/25/1214	6/3/1219	7
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Joukyuu	承久	じょうきゅう	Jōkyū	Zyôkyû	6/3/1219	6/1/1222	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Jouou	貞応	じょうおう	Jōō	Zyôô	6/1/1222	1/7/1225	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Gennin	元仁	げんにん	Gennin	Gennin	1/7/1225	6/4/1125	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Karoku	嘉禄	かろく	Karoku	Karoku	6/4/1125	1/25/1228	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Antei	安貞	あんてい	Antei	Antei	1/25/1228	4/7/1229	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kanki	寛喜	かんき	Kanki	Kanki	4/7/1229	4/30/1232	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Jouei	貞永	じょうえい	Jōei	Zyôei	4/30/1232	6/1/1233	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Tenpuku	天福	てんぷく	Tenpuku	Tenpuku	6/1/1233	12/4/1234	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Bunryaku	文暦	ぶんりゃく	Bunryaku	Bunryaku	12/4/1234	11/8/1235	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Katei	嘉禎	かてい	Katei	Katei	11/8/1235	1/6/1239	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Ryakunin	暦仁	りゃくにん	Ryakunin	Ryakunin	1/6/1239	3/20/1239	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// En'ou	延応	えんおう	En'ō	En'ô	3/20/1239	8/12/1240	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Ninji	仁治	にんじ	Ninji	Ninzi	8/12/1240	3/25/1243	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kangen	寛元	かんげん	Kangen	Kangen	3/25/1243	4/12/1247	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Houji	宝治	ほうじ	Hōji	Hôzi	4/12/1247	5/9/1249	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kenchou	建長	けんちょう	Kenchō	Kentyô	5/9/1249	10/31/1256	8
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kougen	康元	こうげん	Kōgen	Kôgen	10/31/1256	4/7/1257	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Shouka	正嘉	しょうか	Shōka	Syôka	4/7/1257	4/27/1259	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Shougen	正元	しょうげん	Shōgen	Syôgen	4/27/1259	5/31/1260	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Bun'ou	文応	ぶんおう	Bun'ō	Bun'ô	5/31/1260	3/29/1261	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kouchou	弘長	こうちょう	Kōchō	Kôtyô	3/29/1261	4/3/1264	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Bun'ei	文永	ぶんえい	Bun'ei	Bun'ei	4/3/1264	5/29/1275	12
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kenji	建治	けんじ	Kenji	Kenzi	5/29/1275	3/30/1278	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kouan	弘安	こうあん	Kōan	Kôan	3/30/1278	6/5/1288	11
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Shouou	正応	しょうおう	Shōō	Syôô	6/5/1288	9/13/1293	6
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Einin	永仁	えいにん	Einin	Einin	9/13/1293	6/5/1299	7
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Shouan	正安	しょうあん	Shōan	Syôan	6/5/1299	12/18/1302	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kengen	乾元	けんげん	Kengen	Kengen	12/18/1302	9/24/1303	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kagen	嘉元	かげん	Kagen	Kagen	9/24/1303	1/26/1307	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Tokuji	徳治	とくじ	Tokuji	Tokuzi	1/26/1307	11/30/1308	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Engyou	延慶	えんぎょう	Engyō	Engyô	11/30/1308	5/25/1311	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Ouchou	応長	おうちょう	Ōchō	Ôtyô	5/25/1311	5/5/1312	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Shouwa	正和	しょうわ	Shōwa	Syôwa	5/5/1312	3/24/1317	6
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Bunpou	文保	ぶんぽう	Bunpō	Bunpô	3/24/1317	5/26/1319	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Gen'ou	元応	げんおう	Gen'ō	Gen'ô	5/26/1319	3/30/1321	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Genkou	元亨	げんこう	Genkō	Genkô	3/30/1321	1/2/1325	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Shouchuu	正中	しょうちゅう	Shōchū	Syôtyû	1/2/1325	6/5/1326	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Karyaku	嘉暦	かりゃく	Karyaku	Karyaku	6/5/1326	9/30/1329	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Both, NaiveDate::from_ymd(1331, 9, 19), 3));
// Gentoku	元徳	げんとく	Gentoku	Gentoku	9/30/1329	5/31/1332	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::North, None, None));
// Shoukyou	正慶	しょうきょう	Shōkyō	Syôkyô	5/23/1332	7/15/1333	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::South, None, None));
// Genkou	元弘	げんこう	Genkō	Genkô	9/19/1331	3/13/1334	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Both, NaiveDate::from_ymd(1336, 4, 19), 3));
// Kenmu	建武	けんむ	Kenmu	Kenmu	3/13/1334	10/19/1338	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::South, None, None));
// Engen	延元	えんげん	Engen	Engen	4/19/1336	6/2/1340	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::North, None, None));
// Ryakuou	暦応	りゃくおう	Ryakuō	Ryakuô	10/19/1338	6/9/1342	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::South, None, None));
// Koukoku	興国	こうこく	Kōkoku	Kôkoku	6/2/1340	1/28/1347	7
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::North, None, None));
// Kouei	康永	こうえい	Kōei	Kôei	6/9/1342	11/23/1345	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::North, None, None));
// Teiwa	貞和	ていわ	Teiwa	Teiwa	11/23/1345	4/12/1350	6
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::South, None, None));
// Shouhei	正平	しょうへい	Shōhei	Syôhei	1/28/1347	8/24/1370	25
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::North, None, None));
// Kannou	観応	かんのう	Kannō	Kannô	4/12/1350	11/12/1352	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::North, None, None));
// Bunna	文和	ぶんな	Bunna	Bunna	11/12/1352	5/7/1356	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::North, None, None));
// Enbun	延文	えんぶん	Enbun	Enbun	5/7/1356	5/12/1361	6
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::North, None, None));
// Kouan	康安	こうあん	Kōan	Kôan	5/12/1361	10/19/1362	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::North, None, None));
// Jouji	貞治	じょうじ	Jōji	Zyôzi	10/19/1362	3/15/1368	7
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::North, None, None));
// Ouan	応安	おうあん	Ōan	Ôan	3/15/1368	4/6/1375	8
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::South, None, None));
// Kentoku	建徳	けんとく	Kentoku	Kentoku	8/24/1370	6/8/1372*	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::South, None, None));
// Bunchuu	文中	ぶんちゅう	Bunchū	Buntyû	5/9/1372*	7/4/1375	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::North, None, None));
// Eiwa	永和	えいわ	Eiwa	Eiwa	4/6/1375	4/17/1379	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::South, None, None));
// Tenju	天授	てんじゅ	Tenju	Tenzyu	7/4/1375	3/14/1381	7
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::North, None, None));
// Kouryaku	康暦	こうりゃく	Kōryaku	Kôryaku	4/17/1379	3/28/1381	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::South, None, None));
// Kouwa	弘和	こうわ	Kōwa	Kôwa	3/14/1381	5/26/1384	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::North, None, None));
// Eitoku	永徳	えいとく	Eitoku	Eitoku	3/28/1381	3/27/1384	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::North, None, None));
// Shitoku	至徳	しとく	Shitoku	Sitoku	3/27/1384	10/13/1387	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::South, None, None));
// Genchuu	元中	げんちゅう	Genchū	Gentyû	5/26/1384	11/27/1392	9
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::North, None, None));
// Kakei	嘉慶	かけい	Kakei	Kakei	10/13/1387	3/15/1389	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::North, None, None));
// Kouou	康応	こうおう	Kōō	Kôô	3/15/1389	4/20/1390	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Both, None, None));
// Meitoku	明徳	めいとく	Meitoku	Meitoku	4/20/1390	8/10/1394	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Ouei	応永	おうえい	Ōei	Ôei	8/10/1394	6/19/1428	35
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Shouchou	正長	しょうちょう	Shōchō	Syôtyô	6/19/1428	10/12/1429	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Eikyou	永享	えいきょう	Eikyō	Eikyô	10/12/1429	3/19/1441	13
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kakitsu	嘉吉	かきつ	Kakitsu	Kakitu	3/19/1441	3/3/1441	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Bun'an	文安	ぶんあん	Bun'an	Bun'an	3/3/1444	8/25/1449	6
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Houtoku	宝徳	ほうとく	Hōtoku	Hôtoku	8/25/1449	8/19/1452	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kyoutoku	享徳	きょうとく	Kyōtoku	Kyôtoku	8/19/1452	9/15/1455	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Koushou	康正	こうしょう	Kōshō	Kôsyô	9/15/1455	10/25/1457	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Chouroku	長禄	ちょうろく	Chōroku	Tyôroku	10/25/1457	2/10/1461	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kanshou	寛正	かんしょう	Kanshō	Kansyô	2/10/1461	3/23/1466	7
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Bunshou	文正	ぶんしょう	Bunshō	Bunsyô	3/23/1466	4/18/1467	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Ounin	応仁	おうにん	Ōnin	Ônin	4/18/1467	6/17/1469	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Bunmei	文明	ぶんめい	Bunmei	Bunmei	6/17/1469	8/18/1487	19
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Choukyou	長享	ちょうきょう	Chōkyō	Tyôkyô	8/18/1487	9/25/1489	3
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Entoku	延徳	えんとく	Entoku	Entoku	9/25/1489	8/21/1492	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Meiou	明応	めいおう	Meiō	Meiô	8/21/1492	3/28/1501	10
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Bunki	文亀	ぶんき	Bunki	Bunki	3/28/1501	3/26/1504	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Eishou	永正	えいしょう	Eishō	Eisyô	3/26/1504	10/3/1521	18
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Daiei	大永	だいえい	Daiei	Daiei	10/3/1521	9/13/1528	8
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kyouroku	享禄	きょうろく	Kyōroku	Kyôroku	9/13/1528	9/8/1532	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Tenbun	天文	てんぶん	Tenbun	Tenbun	9/8/1532	11/17/1555	24
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kouji	弘治	こうじ	Kōji	Kôzi	11/17/1555	3/28/1558	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Eiroku	永禄	えいろく	Eiroku	Eiroku	3/28/1558	6/6/1570	13
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Genki	元亀	げんき	Genki	Genki	6/6/1570	9/4/1573	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Tenshou	天正	てんしょう	Tenshō	Tensyô	9/4/1573	1/10/1593	20
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Bunroku	文禄	ぶんろく	Bunroku	Bunroku	1/10/1593	12/16/1596	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Keichou	慶長	けいちょう	Keichō	Keityô	12/16/1596	9/5/1615	20
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Gen'na	元和	げんな	Genna	Genna	9/5/1615	4/17/1624	10
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kan'ei	寛永	かんえい	Kan'ei	Kan'ei	4/17/1624	1/13/1645	21
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Shouhou	正保	しょうほう	Shōhō	Syôhô	1/13/1645	4/7/1648	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Keian	慶安	けいあん	Keian	Keian	4/7/1648	10/20/1652	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Jouou	承応	じょうおう	Jōō	Zyôô	10/20/1652	5/18/1655	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Meireki	明暦	めいれき	Meireki	Meireki	5/18/1655	8/21/1658	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Manju	万治	まんじ	Manju	Manzyu	8/21/1658	5/23/1661	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kanbun	寛文	かんぶん	Kanbun	Kanbun	5/23/1661	10/30/1673	13
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Enpou	延宝	えんぽう	Enpō	Enpô	10/30/1673	11/9/1681	9
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Ten'na	天和	てんな	Tenna	Tenna	11/9/1681	4/5/1684	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Joukyou	貞享	じょうきょう	Jōkyō	Zyôkyô	4/5/1684	10/23/1688	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Genroku	元禄	げんろく	Genroku	Genroku	10/23/1688	4/16/1704	17
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Houei	宝永	ほうえい	Hōei	Hôei	4/16/1704	6/11/1711	8
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Shoutoku	正徳	しょうとく	Shōtoku	Syôtoku	6/11/1711	8/9/1716	6
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kyouhou	享保	きょうほう	Kyōhō	Kyôhô	8/9/1716	6/7/1736	21
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Genbun	元文	げんぶん	Genbun	Genbun	6/7/1736	4/12/1741	6
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kanpou	寛保	かんぽう	Kapō	Kapô	4/12/1741	4/13/1744	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Enkyou	延享	えんきょう	Enkyō	Enkyô	4/13/1744	8/5/1748	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kan'en	寛延	かんえん	Kan'en	Kan'en	8/5/1748	12/14/1751	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Houreki	宝暦	ほうれき	Hōreki	Hôreki	12/14/1751	6/30/1764	14
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Meiwa	明和	めいわ	Meiwa	Meiwa	6/30/1764	12/10/1772	9
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// An'ei	安永	あんえい	An'ei	An'ei	12/10/1772	4/25/1781	10
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Tenmei	天明	てんめい	Tenmei	Tenmei	4/25/1781	2/19/1789	9
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kansei	寛政	かんせい	Kansei	Kansei	2/19/1789	3/19/1801	13
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kyouwa	享和	きょうわ	Kyōwa	Kyôwa	3/19/1801	3/22/1804	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Bunka	文化	ぶんか	Bunka	Bunka	3/22/1804	5/26/1818	15
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Bunsei	文政	ぶんせい	Bunsei	Bunsei	5/26/1818	1/23/1831	13
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Tenpou	天保	てんぽう	Tenpō	Tenpô	1/23/1831	1/9/1845	15
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kouka	弘化	こうか	Kōka	Kôka	1/9/1845	4/1/1848	5
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Kaei	嘉永	かえい	Kaei	Kaei	4/1/1848	1/15/1855	7
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Ansei	安政	あんせい	Ansei	Ansei	1/15/1855	4/8/1860	7
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Man'en	万延	まんえん	Man'en	Man'en	4/8/1860	3/29/1861	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Bunkyuu	文久	ぶんきゅう	Bunkyū	Bunkyû	3/29/1861	3/27/1864	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Genji	元治	げんじ	Genji	Genzi	3/27/1864	5/1/1865	2
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Keiou	慶応	けいおう	Keiō	Keiô	5/1/1865	10/23/1868	4
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Meiji	明治	めいじ	Meiji	Meizi	10/23/1868	07/29/1912	45
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Taishou	大正	たいしょう	Taishō	Taisyô	07/30/1912	12/24/1926	15
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Shouwa	昭和	しょうわ	Shōwa	Syôwa	12/25/1926	01/07/1989	64
        m.insert("", Era::new("", "", "", "", "", NaiveDate::from_ymd(), NaiveDate::from_ymd(), , Court::Unified, None, None));
// Heisei	平成	へいせい	Heisei	Heisei	01/08/1989	04/30/2019	31
        m
    };

    static ref ERAS: HashMap<(&Period, &'static str), Era> = {
        let mut m = HashMap::new();
        m.insert((Period::from_str("asuka"), "taika"), Era::new("Taika", "大化", "たいか", "Taika", "Taika", NaiveDate::from_ymd(645, 7, 20), NaiveDate::from_ymd(650, 3, 25), 6, Court::Unified));
        m.insert((Period::from_str("asuka"), "たいか"), Era::new("Taika", "大化", "たいか", "Taika", "Taika", NaiveDate::from_ymd(645, 7, 20), NaiveDate::from_ymd(650, 3, 25), 6, Court::Unified));
        m.insert((Period::from_str("asuka"), "hakuchi"), Era::new("Hakuchi", "白雉", "はくち", "Hakuchi", "Hakuti", NaiveDate::from_ymd(650, 3, 25), NaiveDate::from_ymd(654, 11, 27), 5, Court::Unified));
        m.insert((Period::from_str("asuka"), "hakuti"), Era::new("Hakuchi", "白雉", "はくち", "Hakuchi", "Hakuti", NaiveDate::from_ymd(650, 3, 25), NaiveDate::from_ymd(654, 11, 27), 5, Court::Unified));
        m.insert((Period::from_str("asuka"), "はくち"), Era::new("Hakuchi", "白雉", "はくち", "Hakuchi", "Hakuti", NaiveDate::from_ymd(650, 3, 25), NaiveDate::from_ymd(654, 11, 27), 5, Court::Unified));
        m.insert((Period::from_str("asuka"), "shuchou"), Era::new("Shuchou", "朱鳥", "しゅちょう", "Shuchō", "Syutyô", NaiveDate::from_ymd(686, 8, 17), NaiveDate::from_ymd(686, 10, 4), 1, Court::Unified));
        m.insert((Period::from_str("asuka"), "shucho"), Era::new("Shuchou", "朱鳥", "しゅちょう", "Shuchō", "Syutyô", NaiveDate::from_ymd(686, 8, 17), NaiveDate::from_ymd(686, 10, 4), 1, Court::Unified));
        m.insert((Period::from_str("asuka"), "shuchō"), Era::new("Shuchou", "朱鳥", "しゅちょう", "Shuchō", "Syutyô", NaiveDate::from_ymd(686, 8, 17), NaiveDate::from_ymd(686, 10, 4), 1, Court::Unified));
        m.insert((Period::from_str("asuka"), "syutyô"), Era::new("Shuchou", "朱鳥", "しゅちょう", "Shuchō", "Syutyô", NaiveDate::from_ymd(686, 8, 17), NaiveDate::from_ymd(686, 10, 4), 1, Court::Unified));
        m.insert((Period::from_str("asuka"), "しゅちょう"), Era::new("Shuchou", "朱鳥", "しゅちょう", "Shuchō", "Syutyô", NaiveDate::from_ymd(686, 8, 17), NaiveDate::from_ymd(686, 10, 4), 1, Court::Unified));
        m.insert((Period::from_str("asuka"), "suchou"), Era::new("Shuchou", "朱鳥", "しゅちょう", "Shuchō", "Syutyô", NaiveDate::from_ymd(686, 8, 17), NaiveDate::from_ymd(686, 10, 4), 1, Court::Unified));
        m.insert((Period::from_str("asuka"), "sucho"), Era::new("Shuchou", "朱鳥", "しゅちょう", "Shuchō", "Syutyô", NaiveDate::from_ymd(686, 8, 17), NaiveDate::from_ymd(686, 10, 4), 1, Court::Unified));
        m.insert((Period::from_str("asuka"), "suchō"), Era::new("Shuchou", "朱鳥", "しゅちょう", "Shuchō", "Syutyô", NaiveDate::from_ymd(686, 8, 17), NaiveDate::from_ymd(686, 10, 4), 1, Court::Unified));
        m.insert((Period::from_str("asuka"), "sutyô"), Era::new("Shuchou", "朱鳥", "しゅちょう", "Shuchō", "Syutyô", NaiveDate::from_ymd(686, 8, 17), NaiveDate::from_ymd(686, 10, 4), 1, Court::Unified));
        m.insert((Period::from_str("asuka"), "すちょう"), Era::new("Shuchou", "朱鳥", "しゅちょう", "Shuchō", "Syutyô", NaiveDate::from_ymd(686, 8, 17), NaiveDate::from_ymd(686, 10, 4), 1, Court::Unified));
        m.insert((Period::from_str("asuka"), "akamidori"), Era::new("Shuchou", "朱鳥", "しゅちょう", "Shuchō", "Syutyô", NaiveDate::from_ymd(686, 8, 17), NaiveDate::from_ymd(686, 10, 4), 1, Court::Unified));
        m.insert((Period::from_str("asuka"), "あかみどり"), Era::new("Shuchou", "朱鳥", "しゅちょう", "Shuchō", "Syutyô", NaiveDate::from_ymd(686, 8, 17), NaiveDate::from_ymd(686, 10, 4), 1, Court::Unified));
        m.insert((Period::from_str("asuka"), "taihou"), Era::new("Taihou", "大宝", "たいほう", "Taihō", "Taihô", NaiveDate::from_ymd(701, 5, 7), NaiveDate::from_ymd(704, 6, 20), 4, Court::Unified));
        m.insert((Period::from_str("asuka"), "taiho"), Era::new("Taihou", "大宝", "たいほう", "Taihō", "Taihô", NaiveDate::from_ymd(701, 5, 7), NaiveDate::from_ymd(704, 6, 20), 4, Court::Unified));
        m.insert((Period::from_str("asuka"), "daihou"), Era::new("Taihou", "大宝", "たいほう", "Taihō", "Taihô", NaiveDate::from_ymd(701, 5, 7), NaiveDate::from_ymd(704, 6, 20), 4, Court::Unified));
        m.insert((Period::from_str("asuka"), "daiho"), Era::new("Taihou", "大宝", "たいほう", "Taihō", "Taihô", NaiveDate::from_ymd(701, 5, 7), NaiveDate::from_ymd(704, 6, 20), 4, Court::Unified));
        m.insert((Period::from_str("asuka"), "taihō"), Era::new("Taihou", "大宝", "たいほう", "Taihō", "Taihô", NaiveDate::from_ymd(701, 5, 7), NaiveDate::from_ymd(704, 6, 20), 4, Court::Unified));
        m.insert((Period::from_str("asuka"), "daihō"), Era::new("Taihou", "大宝", "たいほう", "Taihō", "Taihô", NaiveDate::from_ymd(701, 5, 7), NaiveDate::from_ymd(704, 6, 20), 4, Court::Unified));
        m.insert((Period::from_str("asuka"), "taihô"), Era::new("Taihou", "大宝", "たいほう", "Taihō", "Taihô", NaiveDate::from_ymd(701, 5, 7), NaiveDate::from_ymd(704, 6, 20), 4, Court::Unified));
        m.insert((Period::from_str("asuka"), "daihô"), Era::new("Taihou", "大宝", "たいほう", "Taihō", "Taihô", NaiveDate::from_ymd(701, 5, 7), NaiveDate::from_ymd(704, 6, 20), 4, Court::Unified));
        m.insert((Period::from_str("asuka"), "たいほう"), Era::new("Taihou", "大宝", "たいほう", "Taihō", "Taihô", NaiveDate::from_ymd(701, 5, 7), NaiveDate::from_ymd(704, 6, 20), 4, Court::Unified));
        m.insert((Period::from_str("asuka"), "だいほう"), Era::new("Taihou", "大宝", "たいほう", "Taihō", "Taihô", NaiveDate::from_ymd(701, 5, 7), NaiveDate::from_ymd(704, 6, 20), 4, Court::Unified));
        m.insert((Period::from_str("asuka"), "keiun"), Era::new("Keiun", "慶雲", "けいうん", "Keiun", "Keiun", NaiveDate::from_ymd(704, 6, 20), NaiveDate::from_ymd(708, 2, 11), 5, Court::Unified));
        m.insert((Period::from_str("asuka"), "けいうん"), Era::new("Keiun", "慶雲", "けいうん", "Keiun", "Keiun", NaiveDate::from_ymd(704, 6, 20), NaiveDate::from_ymd(708, 2, 11), 5, Court::Unified));
        m.insert((Period::from_str("asuka"), "kyouun"), Era::new("Keiun", "慶雲", "けいうん", "Keiun", "Keiun", NaiveDate::from_ymd(704, 6, 20), NaiveDate::from_ymd(708, 2, 11), 5, Court::Unified));
        m.insert((Period::from_str("asuka"), "kyo'un"), Era::new("Keiun", "慶雲", "けいうん", "Keiun", "Keiun", NaiveDate::from_ymd(704, 6, 20), NaiveDate::from_ymd(708, 2, 11), 5, Court::Unified));
        m.insert((Period::from_str("asuka"), "kyōun"), Era::new("Keiun", "慶雲", "けいうん", "Keiun", "Keiun", NaiveDate::from_ymd(704, 6, 20), NaiveDate::from_ymd(708, 2, 11), 5, Court::Unified));
        m.insert((Period::from_str("asuka"), "kyôun"), Era::new("Keiun", "慶雲", "けいうん", "Keiun", "Keiun", NaiveDate::from_ymd(704, 6, 20), NaiveDate::from_ymd(708, 2, 11), 5, Court::Unified));
        m.insert((Period::from_str("asuka"), "きょううん"), Era::new("Keiun", "慶雲", "けいうん", "Keiun", "Keiun", NaiveDate::from_ymd(704, 6, 20), NaiveDate::from_ymd(708, 2, 11), 5, Court::Unified));
        m.insert((Period::from_str("asuka"), "wadou"), Era::new("Wadou", "和銅", "わどう", "Wadō", "Wadô", NaiveDate::from_ymd(708, 2, 11), NaiveDate::from_ymd(715, 10, 7), 8, Court::Unified));
        m.insert((Period::from_str("asuka"), "wado"), Era::new("Wadou", "和銅", "わどう", "Wadō", "Wadô", NaiveDate::from_ymd(708, 2, 11), NaiveDate::from_ymd(715, 10, 7), 8, Court::Unified));
        m.insert((Period::from_str("asuka"), "wadō"), Era::new("Wadou", "和銅", "わどう", "Wadō", "Wadô", NaiveDate::from_ymd(708, 2, 11), NaiveDate::from_ymd(715, 10, 7), 8, Court::Unified));
        m.insert((Period::from_str("asuka"), "wadô"), Era::new("Wadou", "和銅", "わどう", "Wadō", "Wadô", NaiveDate::from_ymd(708, 2, 11), NaiveDate::from_ymd(715, 10, 7), 8, Court::Unified));
        m.insert((Period::from_str("asuka"), "わどう"), Era::new("Wadou", "和銅", "わどう", "Wadō", "Wadô", NaiveDate::from_ymd(708, 2, 11), NaiveDate::from_ymd(715, 10, 7), 8, Court::Unified));
        m.insert((Period::from_str("nara"), "reiki"), Era::new("Reiki", "霊亀", "れいき", "Reiki", "Reiki", NaiveDate::from_ymd(715, 10, 7), NaiveDate::from_ymd(717, 12, 28), 3, Court::Unified));
        m.insert((Period::from_str("nara"), "れいき"), Era::new("Reiki", "霊亀", "れいき", "Reiki", "Reiki", NaiveDate::from_ymd(715, 10, 7), NaiveDate::from_ymd(717, 12, 28), 3, Court::Unified));
        m
    };
}

#[derive(Clone)]
pub enum Court {
    Unified,
    North,
    South,
    Both,
}

#[derive(Clone, Eq, PartialEq, Hash)]
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
}

impl Era {
    pub fn new(kana_spelling: String, kanji: String, hiragana: String, hepburn: String, iso_3602: String, start_georgian: NaiveDate, end_georgian: NaiveDate, end_year: u32, court: Court) -> Self {
        let era = Era {
            kana_spelling: kana_spelling,
            kanji: kanji,
            hiragana: hiragana,
            hepburn: hepburn,
            iso_3602: iso_3602,
            start_georgian: start_georgian,
            end_georgian: end_georgian,
            end_year: end_year,
            court: court,
        };
        era
    }

    pub fn get(_era: &str, _period: &str) -> Option<Self> {
        unimplemented!()
    }

    pub fn get_from_period(_era: &str, _period: Period) -> Option<Self> {
        unimplemented!()
    }

    pub fn from_kanji(_kanji: &str) -> Option<Self> {
        unimplemented!()
    }

    pub fn from_georgian(_date: NaiveDate) -> Option<Self> {
        unimplemented!()
    }

    pub fn from_georgian_ymd(_year: u32, _month: u32, _day: u32) -> Option<Self> {
        unimplemented!()
    }

    pub fn georgian_start_year(&self) -> i32 {
        self.start_georgian.year()
    }
}

//     //edits to do here
//     //edits to do here
//     //edits to do here
//     ((854, 12, 27), (857, 3, 24), ("Saikou", "", "", "Saikō", "", 4, Court::Unified, None)), //24
//     ((857, 3, 24), (859, 5, 24), ("Ten'an", "", "", "Ten'an", "Ten'an", 3, Court::Unified, None)), //25
//     ((859, 5, 24), (877, 6, 5), ("Jougan", "", "", "Jōgan", "", 19, Court::Unified, None)), //26
//     ((877, 6, 5), (885, 3, 15), ("Gangyou", "", "", "Gangyō", "", 9, Court::Unified, None)), //27
//     ((885, 3, 15), (889, 6, 3), ("Ninna", "", "", "Ninna", "Ninna", 5, Court::Unified, None)), //28
//     ((889, 6, 3), (898, 5, 24), ("Kanpyou", "", "", "Kanpyō", "", 10, Court::Unified, None)), //29
//     ((898, 5, 24), (901, 9, 5), ("Shoutai", "", "", "Shōtai", "", 4, Court::Unified, None)), //30
//     ((901, 9, 5), (923, 6, 3), ("Engi", "", "", "Engi", "Engi", 23, Court::Unified, None)), //31
//     ((923, 6, 3), (931, 5, 21), ("Enchou", "", "", "Enchō", "", 9, Court::Unified, None)), //32
//     ((931, 5, 21), (938, 6, 27), ("Jouhei", "", "", "Jōhei", "", 8, None)), //33
//     ((938, 6, 27), (947, 5, 20), ("Tengyou", "", "", "Tengyō", "", 10, Court::Unified, None)), //34
//     ((947, 5, 20), (957, 11, 26), ("Tenryaku", "", "", "Tenryaku", "Tenryaku", 11, Court::Unified, None)), //35
//     ((957, 11, 26), (961, 3, 10), ("Tentoku", "", "", "Tentoku", "Tentoku", 5, Court::Unified, None)), //36
//     ((961, 3, 10), (964, 8, 24), ("Ouwa", "", "", "", "", 4, Court::Unified, None)), //37
//     ((964, 8, 24), (968, 9, 13), ("Kouhou", "", "", "Kōhō", "", 5, Court::Unified, None)), //38
//     ((968, 9, 13), (970, 5, 8), ("Anna", "", "", "Anna", "Anna", 3, Court::Unified, None)), //39
// ];

// const ERAS: [(Vec<&str>, Vec<(Vec<&str>, Era)>); 14] = [
//     (
//         vec![
//             "asuka", 
//             "asuka jidai", 
//             "asukajidai", 
//             "asuka zidai", 
//             "asukazidai", 
//             "asuka period", 
//             "飛鳥時代", 
//             "飛鳥", 
//             "あすかじだい", 
//             "あすか"
//         ], //period
//         vec![
//             (
//                 vec![
//                     "taika", 
//                     "大化", 
//                     "たいか"
//                 ], 
//                 Era::new(
//                     "Taika", 
//                     "大化", 
//                     "たいか", 
//                     "Taika", 
//                     "Taika", 
//                     NaiveDate::from_ymd(645, 7, 20), 
//                     NaiveDate::from_ymd(650, 3, 25), 
//                     6
//                 )
//             ), //0.era[0] Taika
//             (
//                 vec![
//                     "hakuchi",
//                     "hakuti",
//                     "白雉",
//                     "はくち"
//                 ], 
//                 Era::new(
//                     "Hakuchi", 
//                     "白雉", 
//                     "はくち", 
//                     "Hakuchi", 
//                     "Hakuti", 
//                     NaiveDate::from_ymd(650, 3, 25), 
//                     NaiveDate::from_ymd(654, 11, 27), 
//                     5
//                 )
//             ), //0.era[1] Hakuchi
//             (
//                 vec![
//                     "shuchou",
//                     "shuchō",
//                     "syutyô",
//                     "朱鳥",
//                     "しゅちょう"
//                 ],
//                 Era::new(
//                     "Shuchou", 
//                     "朱鳥", 
//                     "しゅちょう", 
//                     "Shuchō", 
//                     "Syutyô", 
//                     NaiveDate::from_ymd(686, 8, 17), 
//                     NaiveDate::from_ymd(686, 10, 4), 
//                     1
//                 )
//             ), //0.era[2] Shuchou
//             (
//                 vec![
//                     "taihou",
//                     "taihō",
//                     "taihô",
//                     "大宝",
//                     "たいほう"
//                 ],
//                 Era::new(
//                     "Taihou", 
//                     "大宝", 
//                     "たいほう", 
//                     "Taihō", 
//                     "Taihô", 
//                     NaiveDate::from_ymd(701, 5, 7), 
//                     NaiveDate::from_ymd(704, 6, 20), 
//                     4
//                 )
//             ), //0.era[3] Taihou
//             (
//                 vec![
//                     "keiun",
//                     "慶雲",
//                     "けいうん"
//                 ],
//                 Era::new(
//                     "Keiun", 
//                     "慶雲", 
//                     "けいうん", 
//                     "Keiun", 
//                     "Keiun", 
//                     NaiveDate::from_ymd(704, 6, 20), 
//                     NaiveDate::from_ymd(708, 2, 11), 
//                     5
//                 )
//             ), //0.era[4] Keiun
//             (
//                 vec![
//                     "wadou",
//                     "wadō",
//                     "wadô",
//                     "和銅",
//                     "わどう"
//                 ],
//                 Era::new(
//                     "Wadou", 
//                     "和銅", 
//                     "わどう", 
//                     "Wadō", 
//                     "Wadô", 
//                     NaiveDate::from_ymd(708, 2, 11), 
//                     NaiveDate::from_ymd(715, 10, 7), 
//                     8
//                 )
//             ) //0.era[5] Wadou
//         ] //0 eras
//     ), //0 Asuka 538-710
//     (
//         vec![
//             "nara", 
//             "nara jidai", 
//             "narajidai", 
//             "nara zidai", 
//             "narazidai", 
//             "nara period", 
//             "奈良時代", 
//             "奈良", 
//             "ならじだい", 
//             "なら"
//         ], //period
//         vec![
//             (
//                 vec![
//                     "reiki",
//                     "霊亀",
//                     "れいき"
//                 ],
//                 Era::new(
//                     "Reiki", 
//                     "霊亀", 
//                     "れいき", 
//                     "Reiki", 
//                     "Reiki", 
//                     NaiveDate::from_ymd(715, 10, 7), 
//                     NaiveDate::from_ymd(717, 12, 28), 
//                     3
//                 )
//             ), //1.era[0] Reiki
//             (
//                 vec![
//                     "yourou",
//                     "yōrō",
//                     "yôrô",
//                     "養老",
//                     "ようろう"
//                 ],
//                 Era::new(
//                     "Yourou", 
//                     "養老", 
//                     "ようろう", 
//                     "Yōrō", 
//                     "Yôrô", 
//                     NaiveDate::from_ymd(717, 12, 28), 
//                     NaiveDate::from_ymd(724, 3, 27), 
//                     8
//                 )
//             ), //1.era[1] Yourou
//             (
//                 vec![
//                     "jinki",
//                     "zinki",
//                     "神亀",
//                     "じんき"
//                 ],
//                 Era::new(
//                     "Jinki", 
//                     "神亀", 
//                     "じんき", 
//                     "Jinki", 
//                     "Zinki", 
//                     NaiveDate::from_ymd(724, 3, 27), 
//                     NaiveDate::from_ymd(729, 9, 6), 
//                     6
//                 )
//             ), //1.era[2] Jinki
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[3] Tenpyou
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[4] Tenpyou-Kanpou
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[5] Tenpyou-Shouhou
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[6] Tenpyou-Houji
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[7] Tenpyou-Jingo
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[8] Jingo-Keiun
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[9] Houki
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[10] Ten'ou
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[11] Enryaku
//         ] //1 eras
//     ), //1 Nara 710-794
//     (
//         vec![
//             "heian", 
//             "heian jidai", 
//             "heianjidai", 
//             "heian zidai", 
//             "heianzidai", 
//             "heian period", 
//             "平安時代", 
//             "平安", 
//             "へいあんじだい", 
//             "へいあん"
//         ], //period
//         vec![
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //2.era[0-87]
//         ] //eras
//     ), //2 Heian 794-1185
//     (
//         vec![
//             "kamakura", 
//             "kamakura jidai", 
//             "kamakurajidai", 
//             "kamakura zidai", 
//             "kamakurazidai", 
//             "kamakura period", 
//             "鎌倉時代", 
//             "鎌倉", 
//             "かまくらじだい", 
//             "かまくら"
//         ], //period
//         vec![
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //3.era[0-50]
//         ]
//     ), //3 Kamakura 1185-1333
//     (
//         vec![
//             "kenmu no shinsei", 
//             "kenmunoshinsei", 
//             "kenmu", 
//             "kenmu no sinsei", 
//             "kenmunosinsei", 
//             "kenmu restoration", 
//             "建武の新政", 
//             "建武", 
//             "けんむのしんせい", 
//             "けんむ"
//         ], //period
//         vec![
//             (
//                 vec![
//                     "kenmu"
//                 ],
//                 Era
//             ) //4.era[0] Kenmu
//         ]
//     ), //4 Kenmu no shinsei 1333-1336
//     (
//         vec![
//             "muromachi", 
//             "muromachi jidai", 
//             "muromachijidai", 
//             "muromati", 
//             "muromati zidai", 
//             "muromatizidai", 
//             "muromachi period", 
//             "muromati period", 
//             "室町時代", 
//             "室町", 
//             "むろまちじだい", 
//             "むろまち",
//             "ashikaga", 
//             "ashikaga jidai", 
//             "ashikagajidai", 
//             "asikaga", 
//             "asikaga zidai", 
//             "asikagazidai", 
//             "ashikaga period", 
//             "asikaga period", 
//             "足利時代", 
//             "足利", 
//             "あしかがじだい", 
//             "あしかが"
//         ], //period
//         vec![
//             (
//                 vec![
//                     ""
//                 ],
//                 Era
//             ) //5.era[0-47] 
//         ]
//     ), //5 Muromachi/Ashikaga 1336-1573
//     (
//         vec![
//             "nanboku-chou", 
//             "nanboku-chou jidai", 
//             "nanbokuchou", 
//             "nanbokuchoujidai", 
//             "nanboku-chō", 
//             "nanboku-chō jidai", 
//             "nanbokuchō", 
//             "nanbokuchō jidai", 
//             "nanbokuchōjidai", 
//             "nanboku-tyô", 
//             "nanboku-tyô zidai", 
//             "nanbokutyô", 
//             "nanbokutyôzidai", 
//             "south and north courts period", 
//             "northern and southern courts period", 
//             "南北朝時代", 
//             "南北朝", 
//             "なんぼくちょうじだい", 
//             "なんぼくちょう"
//         ], //periods
//         vec![
//             (
//                 vec![
//                     ""
//                 ],
//                 Era
//             ), //6.era[0-23]
//         ] //6 eras
//     ), //6 Nanboku-chou 1336-1392
//     (
//         vec![
//             "sengoku", 
//             "sengoku jidai", 
//             "sengokujidai", 
//             "sengoku zidai", 
//             "sengokuzidai", 
//             "sengoku period", 
//             "warring states period", 
//             "戦国時代", 
//             "戦国", 
//             "せんごくじだい", 
//             "せんごく"
//         ], //period
//         vec![
//             (
//                 vec![
//                     ""
//                 ],
//                 Era
//             ), //7.era[0-12]
//         ] //7 eras
//     ), //7 Sengoku 1467-1568
//     (
//         vec![
//             "aduchi-momoyama", 
//             "aduchi-momoyama jidai", 
//             "aduchimomoyama", 
//             "aduchimomoyama jidai", 
//             "aduchimomoyamajidai", 
//             "azuchi-momoyama", 
//             "azuchi-momoyama jidai", 
//             "azuchimomoyama", 
//             "azuchimomoyamajidai", 
//             "azuti-momoyama", 
//             "azuti-momoyama zidai", 
//             "azutimomoyama", 
//             "azutimomoyama zidai", 
//             "azutimomoyamazidai", 
//             "aduti-momoyama", 
//             "aduti-momoyama zidai", 
//             "adutimomoyama", 
//             "adutimomoyama zidai", 
//             "adutimomoyamazidai", 
//             "azuchi-momoyama period", 
//             "azuchimomoyama period", 
//             "azuti-momoyama period", 
//             "azutimomoyama period", 
//             "aduti-momoyama period", 
//             "adutimomoyama period", 
//             "安土桃山時代", 
//             "安土桃山", 
//             "あづちももやまじだい", 
//             "あづちももやま"
//         ], //period
//         vec![
//             (
//                 vec![
//                     ""
//                 ],
//                 Era
//             ) //8.era[0-2]
//         ] //8 eras
//     ), //8 Azuchi-Momoyama 1573-1603
//     (
//         vec![
//             "edo", 
//             "edo jidai", 
//             "edojidai", 
//             "edo zidai", 
//             "edozidai", 
//             "edo period", 
//             "江戸時代", 
//             "江戸", 
//             "えどじだい", 
//             "えど",
//             "tokugawa", 
//             "tokugawa jidai", 
//             "tokugawajidai", 
//             "tokugawa zidai", 
//             "tokugawazidai", 
//             "tokugawa period", 
//             "徳川時代", 
//             "徳川", 
//             "とくがわじだい", 
//             "とくがわ"
//         ], //period
//         vec![
//             (
//                 vec![
//                     ""
//                 ],
//                 Era
//             ), //9.era[0-34]
//         ] //eras
//     ), //9 Edo/Tokugawa 1603-1868
//     (
//         vec![
//             "meiji", 
//             "meiji jidai", 
//             "meijijidai", 
//             "meizi", 
//             "meizi zidai", 
//             "meizizidai", 
//             "meiji period", 
//             "meizi period", 
//             "meiji era", 
//             "meizi era", 
//             "明治時代", 
//             "明治", 
//             "めいじじだい", 
//             "めいじ", 
//             "㍾"
//         ], //period
//         vec![
//             (
//                 vec![
//                     "meiji",
//                     "meizi",
//                     "meiji era",
//                     "meizi era",
//                     "明治",
//                     "めいじ",
//                     "㍾"
//                 ],
//                 Era::new(
//                     "Meiji",
//                     "明治",
//                     "めいじ",
//                     "Meiji",
//                     "Meizi",
//                     NaiveDate::from_ymd(1868, 10, 23),
//                     NaiveDate::from_ymd(1912, 7, 29),
//                     45
//                 )
//             ) //10.era[0] Meiji
//         ] //10 eras
//     ), //10 Meiji 1868-1912
//     (
//         vec![
//             "",
//         ], //period
//         vec![
//             (
//                 vec![
//                     ""
//                 ],
//                 Era::new(
//                     "",
//                     "",
//                     "",
//                     "",
//                     "",
//                     NaiveDate::from_ymd(1912, 7, 29),
//                     NaiveDate::from_ymd(, , ),
//                     15
//                 )
//             ) //11.era[0] Taishou
//         ] //11 eras
//     ), //11 Taishou 1912-1926
//     (
//         vec![
//             "",
//         ], //period
//         vec![
//             (
//                 vec![
//                     ""
//                 ],
//                 Era::new(
//                     "",
//                     "",
//                     "",
//                     "",
//                     "",
//                     NaiveDate::from_ymd(, , ),
//                     NaiveDate::from_ymd(, , ),
//                     0
//                 )
//             ) //12.era[0] Shouwa
//         ] //12 eras
//     ), //12 Shouwa 1926-1989
//     (
//         vec![
//             "",
//         ], //period
//         vec![
//             (
//                 vec![
//                     ""
//                 ],
//                 Era::new(
//                     "",
//                     "",
//                     "",
//                     "",
//                     "",
//                     NaiveDate::from_ymd(, , ),
//                     NaiveDate::from_ymd(, , ),
//                     0
//                 )
//             ) //13.era[0] Heisei
//         ] //13 eras
//     ), //13 Heisei 1989-2019
// ];