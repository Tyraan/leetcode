
func MinWindow(s string, t string) string {

	targetCounter := make(map[int32]uint32)
	for _, char := range t {
		targetCounter[char] += 1
	}

	counter := make(map[int32]uint32)
	for _, char := range s {
		if _, exist := targetCounter[char]; exist {
			counter[char] += 1
		}
	}

	for idx, count := range counter {
		targetCount, exist := targetCounter[idx]
		if !exist || count < targetCount {
			return ""
		}
	}
	//leftIdx, rightIdx, 
	return ""

	//for i, char := range s {
	//	if _, exist := targetCharMap[char]; exist {
	//		for _, runeList := range idxMap {
	//			runeList = append(runeList, char)
	//		}
	//
	//		idxMap[i] = []rune{char}
	//	}
	//}

}
