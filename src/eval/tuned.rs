// K = 0.858
// Mean error = 0.13120701962868858

use super::{weight::Weight, EvalParams};
impl Default for EvalParams { fn default() -> Self { EvalParams { pst_mid: [Weight::new(82).into(), Weight::new(82).into(), Weight::new(82).into(), Weight::new(82).into(), Weight::new(82).into(), Weight::new(82).into(), Weight::new(82).into(), Weight::new(82).into(), Weight::new(49).into(), Weight::new(87).into(), Weight::new(67).into(), Weight::new(56).into(), Weight::new(62).into(), Weight::new(94).into(), Weight::new(129).into(), Weight::new(70).into(), Weight::new(59).into(), Weight::new(78).into(), Weight::new(79).into(), Weight::new(69).into(), Weight::new(82).into(), Weight::new(61).into(), Weight::new(113).into(), Weight::new(71).into(), Weight::new(51).into(), Weight::new(83).into(), Weight::new(76).into(), Weight::new(88).into(), Weight::new(95).into(), Weight::new(89).into(), Weight::new(93).into(), Weight::new(47).into(), Weight::new(67).into(), Weight::new(88).into(), Weight::new(81).into(), Weight::new(90).into(), Weight::new(101).into(), Weight::new(94).into(), Weight::new(98).into(), Weight::new(59).into(), Weight::new(76).into(), Weight::new(89).into(), Weight::new(108).into(), Weight::new(113).into(), Weight::new(147).into(), Weight::new(138).into(), Weight::new(107).into(), Weight::new(62).into(), Weight::new(180).into(), Weight::new(216).into(), Weight::new(143).into(), Weight::new(177).into(), Weight::new(150).into(), Weight::new(208).into(), Weight::new(116).into(), Weight::new(71).into(), Weight::new(82).into(), Weight::new(82).into(), Weight::new(82).into(), Weight::new(82).into(), Weight::new(82).into(), Weight::new(82).into(), Weight::new(82).into(), Weight::new(82).into(), Weight::new(232).into(), Weight::new(323).into(), Weight::new(279).into(), Weight::new(304).into(), Weight::new(320).into(), Weight::new(309).into(), Weight::new(321).into(), Weight::new(314).into(), Weight::new(308).into(), Weight::new(284).into(), Weight::new(325).into(), Weight::new(342).into(), Weight::new(336).into(), Weight::new(355).into(), Weight::new(323).into(), Weight::new(318).into(), Weight::new(314).into(), Weight::new(328).into(), Weight::new(353).into(), Weight::new(347).into(), Weight::new(356).into(), Weight::new(351).into(), Weight::new(362).into(), Weight::new(321).into(), Weight::new(325).into(), Weight::new(341).into(), Weight::new(353).into(), Weight::new(352).into(), Weight::new(365).into(), Weight::new(356).into(), Weight::new(358).into(), Weight::new(324).into(), Weight::new(328).into(), Weight::new(349).into(), Weight::new(356).into(), Weight::new(390).into(), Weight::new(371).into(), Weight::new(406).into(), Weight::new(348).into(), Weight::new(359).into(), Weight::new(290).into(), Weight::new(397).into(), Weight::new(374).into(), Weight::new(402).into(), Weight::new(421).into(), Weight::new(466).into(), Weight::new(410).into(), Weight::new(381).into(), Weight::new(264).into(), Weight::new(296).into(), Weight::new(409).into(), Weight::new(373).into(), Weight::new(360).into(), Weight::new(399).into(), Weight::new(344).into(), Weight::new(320).into(), Weight::new(170).into(), Weight::new(248).into(), Weight::new(303).into(), Weight::new(288).into(), Weight::new(398).into(), Weight::new(240).into(), Weight::new(322).into(), Weight::new(230).into(), Weight::new(332).into(), Weight::new(362).into(), Weight::new(367).into(), Weight::new(344).into(), Weight::new(352).into(), Weight::new(353).into(), Weight::new(326).into(), Weight::new(344).into(), Weight::new(369).into(), Weight::new(397).into(), Weight::new(381).into(), Weight::new(372).into(), Weight::new(375).into(), Weight::new(386).into(), Weight::new(408).into(), Weight::new(366).into(), Weight::new(365).into(), Weight::new(380).into(), Weight::new(387).into(), Weight::new(383).into(), Weight::new(379).into(), Weight::new(391).into(), Weight::new(380).into(), Weight::new(375).into(), Weight::new(359).into(), Weight::new(378).into(), Weight::new(372).into(), Weight::new(391).into(), Weight::new(399).into(), Weight::new(382).into(), Weight::new(375).into(), Weight::new(369).into(), Weight::new(361).into(), Weight::new(369).into(), Weight::new(384).into(), Weight::new(415).into(), Weight::new(402).into(), Weight::new(402).into(), Weight::new(378).into(), Weight::new(363).into(), Weight::new(349).into(), Weight::new(402).into(), Weight::new(408).into(), Weight::new(405).into(), Weight::new(400).into(), Weight::new(415).into(), Weight::new(402).into(), Weight::new(363).into(), Weight::new(339).into(), Weight::new(381).into(), Weight::new(347).into(), Weight::new(352).into(), Weight::new(395).into(), Weight::new(424).into(), Weight::new(383).into(), Weight::new(318).into(), Weight::new(336).into(), Weight::new(369).into(), Weight::new(283).into(), Weight::new(328).into(), Weight::new(340).into(), Weight::new(323).into(), Weight::new(372).into(), Weight::new(357).into(), Weight::new(464).into(), Weight::new(467).into(), Weight::new(479).into(), Weight::new(488).into(), Weight::new(489).into(), Weight::new(480).into(), Weight::new(440).into(), Weight::new(471).into(), Weight::new(433).into(), Weight::new(461).into(), Weight::new(457).into(), Weight::new(468).into(), Weight::new(476).into(), Weight::new(488).into(), Weight::new(471).into(), Weight::new(406).into(), Weight::new(432).into(), Weight::new(452).into(), Weight::new(461).into(), Weight::new(460).into(), Weight::new(480).into(), Weight::new(477).into(), Weight::new(472).into(), Weight::new(444).into(), Weight::new(441).into(), Weight::new(451).into(), Weight::new(465).into(), Weight::new(476).into(), Weight::new(486).into(), Weight::new(470).into(), Weight::new(483).into(), Weight::new(454).into(), Weight::new(453).into(), Weight::new(466).into(), Weight::new(484).into(), Weight::new(503).into(), Weight::new(501).into(), Weight::new(512).into(), Weight::new(469).into(), Weight::new(457).into(), Weight::new(472).into(), Weight::new(496).into(), Weight::new(503).into(), Weight::new(513).into(), Weight::new(494).into(), Weight::new(522).into(), Weight::new(538).into(), Weight::new(493).into(), Weight::new(504).into(), Weight::new(509).into(), Weight::new(535).into(), Weight::new(539).into(), Weight::new(557).into(), Weight::new(544).into(), Weight::new(503).into(), Weight::new(521).into(), Weight::new(509).into(), Weight::new(519).into(), Weight::new(509).into(), Weight::new(528).into(), Weight::new(540).into(), Weight::new(486).into(), Weight::new(508).into(), Weight::new(520).into(), Weight::new(1024).into(), Weight::new(1007).into(), Weight::new(1016).into(), Weight::new(1032).into(), Weight::new(1010).into(), Weight::new(1000).into(), Weight::new(994).into(), Weight::new(975).into(), Weight::new(990).into(), Weight::new(1017).into(), Weight::new(1036).into(), Weight::new(1029).into(), Weight::new(1031).into(), Weight::new(1040).into(), Weight::new(1022).into(), Weight::new(1026).into(), Weight::new(1011).into(), Weight::new(1028).into(), Weight::new(1014).into(), Weight::new(1025).into(), Weight::new(1020).into(), Weight::new(1026).into(), Weight::new(1039).into(), Weight::new(1030).into(), Weight::new(1013).into(), Weight::new(999).into(), Weight::new(1016).into(), Weight::new(1013).into(), Weight::new(1023).into(), Weight::new(1021).into(), Weight::new(1028).into(), Weight::new(1022).into(), Weight::new(998).into(), Weight::new(998).into(), Weight::new(1009).into(), Weight::new(1009).into(), Weight::new(1024).into(), Weight::new(1042).into(), Weight::new(1023).into(), Weight::new(1026).into(), Weight::new(1012).into(), Weight::new(1008).into(), Weight::new(1032).into(), Weight::new(1033).into(), Weight::new(1054).into(), Weight::new(1081).into(), Weight::new(1072).into(), Weight::new(1082).into(), Weight::new(1001).into(), Weight::new(986).into(), Weight::new(1020).into(), Weight::new(1026).into(), Weight::new(1009).into(), Weight::new(1082).into(), Weight::new(1053).into(), Weight::new(1079).into(), Weight::new(997).into(), Weight::new(1025).into(), Weight::new(1054).into(), Weight::new(1037).into(), Weight::new(1084).into(), Weight::new(1069).into(), Weight::new(1068).into(), Weight::new(1070).into(), Weight::new(-15).into(), Weight::new(36).into(), Weight::new(12).into(), Weight::new(-54).into(), Weight::new(14).into(), Weight::new(-28).into(), Weight::new(24).into(), Weight::new(14).into(), Weight::new(1).into(), Weight::new(7).into(), Weight::new(-8).into(), Weight::new(-64).into(), Weight::new(-43).into(), Weight::new(-16).into(), Weight::new(9).into(), Weight::new(8).into(), Weight::new(-14).into(), Weight::new(-14).into(), Weight::new(-22).into(), Weight::new(-46).into(), Weight::new(-44).into(), Weight::new(-30).into(), Weight::new(-15).into(), Weight::new(-27).into(), Weight::new(-49).into(), Weight::new(-1).into(), Weight::new(-27).into(), Weight::new(-39).into(), Weight::new(-46).into(), Weight::new(-44).into(), Weight::new(-33).into(), Weight::new(-51).into(), Weight::new(-17).into(), Weight::new(-20).into(), Weight::new(-12).into(), Weight::new(-27).into(), Weight::new(-30).into(), Weight::new(-25).into(), Weight::new(-14).into(), Weight::new(-36).into(), Weight::new(-9).into(), Weight::new(24).into(), Weight::new(2).into(), Weight::new(-16).into(), Weight::new(-20).into(), Weight::new(6).into(), Weight::new(22).into(), Weight::new(-22).into(), Weight::new(29).into(), Weight::new(-1).into(), Weight::new(-20).into(), Weight::new(-7).into(), Weight::new(-8).into(), Weight::new(-4).into(), Weight::new(-38).into(), Weight::new(-29).into(), Weight::new(-65).into(), Weight::new(23).into(), Weight::new(16).into(), Weight::new(-15).into(), Weight::new(-56).into(), Weight::new(-34).into(), Weight::new(2).into(), Weight::new(13).into()], pst_end: [Weight::new(94).into(), Weight::new(94).into(), Weight::new(94).into(), Weight::new(94).into(), Weight::new(94).into(), Weight::new(94).into(), Weight::new(94).into(), Weight::new(94).into(), Weight::new(107).into(), Weight::new(105).into(), Weight::new(104).into(), Weight::new(104).into(), Weight::new(107).into(), Weight::new(94).into(), Weight::new(84).into(), Weight::new(84).into(), Weight::new(98).into(), Weight::new(101).into(), Weight::new(88).into(), Weight::new(95).into(), Weight::new(94).into(), Weight::new(91).into(), Weight::new(93).into(), Weight::new(88).into(), Weight::new(107).into(), Weight::new(103).into(), Weight::new(87).into(), Weight::new(87).into(), Weight::new(86).into(), Weight::new(84).into(), Weight::new(95).into(), Weight::new(93).into(), Weight::new(126).into(), Weight::new(118).into(), Weight::new(107).into(), Weight::new(99).into(), Weight::new(92).into(), Weight::new(98).into(), Weight::new(111).into(), Weight::new(111).into(), Weight::new(188).into(), Weight::new(194).into(), Weight::new(179).into(), Weight::new(161).into(), Weight::new(150).into(), Weight::new(147).into(), Weight::new(176).into(), Weight::new(178).into(), Weight::new(272).into(), Weight::new(267).into(), Weight::new(252).into(), Weight::new(228).into(), Weight::new(241).into(), Weight::new(226).into(), Weight::new(259).into(), Weight::new(281).into(), Weight::new(94).into(), Weight::new(94).into(), Weight::new(94).into(), Weight::new(94).into(), Weight::new(94).into(), Weight::new(94).into(), Weight::new(94).into(), Weight::new(94).into(), Weight::new(252).into(), Weight::new(230).into(), Weight::new(258).into(), Weight::new(266).into(), Weight::new(259).into(), Weight::new(263).into(), Weight::new(231).into(), Weight::new(217).into(), Weight::new(239).into(), Weight::new(261).into(), Weight::new(271).into(), Weight::new(276).into(), Weight::new(279).into(), Weight::new(261).into(), Weight::new(258).into(), Weight::new(237).into(), Weight::new(258).into(), Weight::new(278).into(), Weight::new(280).into(), Weight::new(296).into(), Weight::new(291).into(), Weight::new(279).into(), Weight::new(261).into(), Weight::new(259).into(), Weight::new(263).into(), Weight::new(275).into(), Weight::new(297).into(), Weight::new(306).into(), Weight::new(297).into(), Weight::new(298).into(), Weight::new(285).into(), Weight::new(263).into(), Weight::new(264).into(), Weight::new(284).into(), Weight::new(303).into(), Weight::new(303).into(), Weight::new(303).into(), Weight::new(292).into(), Weight::new(289).into(), Weight::new(263).into(), Weight::new(257).into(), Weight::new(261).into(), Weight::new(291).into(), Weight::new(290).into(), Weight::new(280).into(), Weight::new(272).into(), Weight::new(262).into(), Weight::new(240).into(), Weight::new(256).into(), Weight::new(273).into(), Weight::new(256).into(), Weight::new(279).into(), Weight::new(272).into(), Weight::new(256).into(), Weight::new(257).into(), Weight::new(229).into(), Weight::new(223).into(), Weight::new(243).into(), Weight::new(268).into(), Weight::new(253).into(), Weight::new(250).into(), Weight::new(254).into(), Weight::new(218).into(), Weight::new(182).into(), Weight::new(274).into(), Weight::new(288).into(), Weight::new(271).into(), Weight::new(292).into(), Weight::new(288).into(), Weight::new(281).into(), Weight::new(292).into(), Weight::new(280).into(), Weight::new(283).into(), Weight::new(279).into(), Weight::new(290).into(), Weight::new(296).into(), Weight::new(301).into(), Weight::new(288).into(), Weight::new(282).into(), Weight::new(270).into(), Weight::new(285).into(), Weight::new(294).into(), Weight::new(305).into(), Weight::new(307).into(), Weight::new(310).into(), Weight::new(300).into(), Weight::new(290).into(), Weight::new(282).into(), Weight::new(291).into(), Weight::new(300).into(), Weight::new(310).into(), Weight::new(316).into(), Weight::new(304).into(), Weight::new(307).into(), Weight::new(294).into(), Weight::new(288).into(), Weight::new(294).into(), Weight::new(306).into(), Weight::new(309).into(), Weight::new(306).into(), Weight::new(311).into(), Weight::new(307).into(), Weight::new(300).into(), Weight::new(299).into(), Weight::new(299).into(), Weight::new(289).into(), Weight::new(297).into(), Weight::new(296).into(), Weight::new(295).into(), Weight::new(303).into(), Weight::new(297).into(), Weight::new(301).into(), Weight::new(289).into(), Weight::new(293).into(), Weight::new(304).into(), Weight::new(285).into(), Weight::new(294).into(), Weight::new(284).into(), Weight::new(293).into(), Weight::new(283).into(), Weight::new(283).into(), Weight::new(276).into(), Weight::new(286).into(), Weight::new(289).into(), Weight::new(290).into(), Weight::new(288).into(), Weight::new(280).into(), Weight::new(273).into(), Weight::new(504).into(), Weight::new(514).into(), Weight::new(515).into(), Weight::new(511).into(), Weight::new(507).into(), Weight::new(505).into(), Weight::new(516).into(), Weight::new(491).into(), Weight::new(506).into(), Weight::new(506).into(), Weight::new(512).into(), Weight::new(514).into(), Weight::new(503).into(), Weight::new(503).into(), Weight::new(501).into(), Weight::new(509).into(), Weight::new(508).into(), Weight::new(512).into(), Weight::new(507).into(), Weight::new(511).into(), Weight::new(505).into(), Weight::new(500).into(), Weight::new(504).into(), Weight::new(496).into(), Weight::new(515).into(), Weight::new(517).into(), Weight::new(520).into(), Weight::new(516).into(), Weight::new(507).into(), Weight::new(506).into(), Weight::new(504).into(), Weight::new(501).into(), Weight::new(516).into(), Weight::new(515).into(), Weight::new(525).into(), Weight::new(513).into(), Weight::new(514).into(), Weight::new(513).into(), Weight::new(511).into(), Weight::new(514).into(), Weight::new(519).into(), Weight::new(519).into(), Weight::new(519).into(), Weight::new(517).into(), Weight::new(516).into(), Weight::new(509).into(), Weight::new(507).into(), Weight::new(509).into(), Weight::new(523).into(), Weight::new(525).into(), Weight::new(525).into(), Weight::new(523).into(), Weight::new(509).into(), Weight::new(515).into(), Weight::new(520).into(), Weight::new(515).into(), Weight::new(525).into(), Weight::new(522).into(), Weight::new(530).into(), Weight::new(527).into(), Weight::new(524).into(), Weight::new(524).into(), Weight::new(520).into(), Weight::new(517).into(), Weight::new(903).into(), Weight::new(908).into(), Weight::new(914).into(), Weight::new(893).into(), Weight::new(931).into(), Weight::new(904).into(), Weight::new(916).into(), Weight::new(895).into(), Weight::new(914).into(), Weight::new(913).into(), Weight::new(906).into(), Weight::new(920).into(), Weight::new(920).into(), Weight::new(913).into(), Weight::new(900).into(), Weight::new(904).into(), Weight::new(920).into(), Weight::new(909).into(), Weight::new(951).into(), Weight::new(942).into(), Weight::new(945).into(), Weight::new(953).into(), Weight::new(946).into(), Weight::new(941).into(), Weight::new(918).into(), Weight::new(964).into(), Weight::new(955).into(), Weight::new(983).into(), Weight::new(967).into(), Weight::new(970).into(), Weight::new(975).into(), Weight::new(959).into(), Weight::new(939).into(), Weight::new(958).into(), Weight::new(960).into(), Weight::new(981).into(), Weight::new(993).into(), Weight::new(976).into(), Weight::new(993).into(), Weight::new(972).into(), Weight::new(916).into(), Weight::new(942).into(), Weight::new(945).into(), Weight::new(985).into(), Weight::new(983).into(), Weight::new(971).into(), Weight::new(955).into(), Weight::new(945).into(), Weight::new(919).into(), Weight::new(956).into(), Weight::new(968).into(), Weight::new(977).into(), Weight::new(994).into(), Weight::new(961).into(), Weight::new(966).into(), Weight::new(936).into(), Weight::new(927).into(), Weight::new(958).into(), Weight::new(958).into(), Weight::new(963).into(), Weight::new(963).into(), Weight::new(955).into(), Weight::new(946).into(), Weight::new(956).into(), Weight::new(-53).into(), Weight::new(-34).into(), Weight::new(-21).into(), Weight::new(-11).into(), Weight::new(-32).into(), Weight::new(-14).into(), Weight::new(-26).into(), Weight::new(-43).into(), Weight::new(-27).into(), Weight::new(-11).into(), Weight::new(4).into(), Weight::new(13).into(), Weight::new(14).into(), Weight::new(4).into(), Weight::new(-5).into(), Weight::new(-17).into(), Weight::new(-19).into(), Weight::new(-3).into(), Weight::new(11).into(), Weight::new(21).into(), Weight::new(23).into(), Weight::new(16).into(), Weight::new(7).into(), Weight::new(-9).into(), Weight::new(-18).into(), Weight::new(-4).into(), Weight::new(21).into(), Weight::new(24).into(), Weight::new(27).into(), Weight::new(23).into(), Weight::new(9).into(), Weight::new(-11).into(), Weight::new(-8).into(), Weight::new(22).into(), Weight::new(24).into(), Weight::new(27).into(), Weight::new(26).into(), Weight::new(33).into(), Weight::new(26).into(), Weight::new(3).into(), Weight::new(10).into(), Weight::new(17).into(), Weight::new(23).into(), Weight::new(15).into(), Weight::new(20).into(), Weight::new(45).into(), Weight::new(44).into(), Weight::new(13).into(), Weight::new(-12).into(), Weight::new(17).into(), Weight::new(14).into(), Weight::new(17).into(), Weight::new(17).into(), Weight::new(38).into(), Weight::new(23).into(), Weight::new(11).into(), Weight::new(-74).into(), Weight::new(-35).into(), Weight::new(-18).into(), Weight::new(-18).into(), Weight::new(-11).into(), Weight::new(15).into(), Weight::new(4).into(), Weight::new(-17).into()], rook_open_file_bonus: Weight::new(18).into(), unshield_king_penalty: Weight::new(-19).into() } } }
