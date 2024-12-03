package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
)

type packet struct {
	valueType string
	value     int
}

var (
	versionSum int
	packetList []packet
)

func main() {

	hexToBits := make(map[byte]string, 0)
	hexToBits['0'] = "0000"
	hexToBits['1'] = "0001"
	hexToBits['2'] = "0010"
	hexToBits['3'] = "0011"
	hexToBits['4'] = "0100"
	hexToBits['5'] = "0101"
	hexToBits['6'] = "0110"
	hexToBits['7'] = "0111"
	hexToBits['8'] = "1000"
	hexToBits['9'] = "1001"
	hexToBits['A'] = "1010"
	hexToBits['B'] = "1011"
	hexToBits['C'] = "1100"
	hexToBits['D'] = "1101"
	hexToBits['E'] = "1110"
	hexToBits['F'] = "1111"

	file, err := os.Open("d16/puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		entry := scanner.Text()
		var packets string

		for i := 0; i < len(entry); i++ {
			packets += hexToBits[entry[i]]
		}
		_, result := processPacket(packets)
		fmt.Printf("Result %v\n", result)
	}
}

func processPacket(packets string) (int, int) {
	//fmt.Printf("Packets %v\n", packets)
	//fmt.Printf("Packets bit length %v\n", len(packets))
	bitsProcessed := 0
	packet_version, _ := strconv.ParseInt(packets[0:3], 2, 64)
	bitsProcessed += 3
	packet_type_id, _ := strconv.ParseInt(packets[3:6], 2, 64)
	bitsProcessed += 3
	//fmt.Printf("packet_version %v\n", packet_version)
	versionSum += int(packet_version)
	fmt.Printf("packet_type_id %v\n", packet_type_id)

	queue := make([]int, 0)
	var number string
	if packet_type_id == 4 {
		// literal value
		for len(packets[bitsProcessed:]) > 0 {
			if packets[bitsProcessed:][0] == '1' {
				number += packets[bitsProcessed+1 : bitsProcessed+5]
				bitsProcessed += 5
			} else if packets[bitsProcessed:][0] == '0' {
				// last number
				number += packets[bitsProcessed+1 : bitsProcessed+5]
				//fmt.Printf("number %v\n", number)
				number_decimal, _ := strconv.ParseInt(number, 2, 64)
				fmt.Printf("number_decimal %v\n", number_decimal)
				newPacket := packet{
					valueType: "number_decimal",
					value:     int(number_decimal),
				}
				packetList = append(packetList, newPacket)
				bitsProcessed += 5
				return bitsProcessed, int(number_decimal)
			}
		}

	} else {
		// operator
		newPacket := packet{
			valueType: "packet_type_id",
			value:     int(packet_type_id),
		}
		packetList = append(packetList, newPacket)

		length_type_id := packets[bitsProcessed:][0]
		bitsProcessed++
		if length_type_id == '0' {
			total_length_in_bits, _ := strconv.ParseInt(packets[bitsProcessed:bitsProcessed+15], 2, 64)
			//fmt.Printf("total_length_in_bits %v\n", total_length_in_bits)
			bitsProcessed += 15

			for total_length_in_bits > 0 {
				bitsProcessedTemp, tempValue := processPacket(packets[bitsProcessed:])
				queue = append(queue, tempValue)
				total_length_in_bits -= int64(bitsProcessedTemp)
				bitsProcessed += bitsProcessedTemp
			}

		} else {
			number_of_subpackets, _ := strconv.ParseInt(packets[bitsProcessed:bitsProcessed+11], 2, 64)
			//fmt.Printf("number_of_subpackets %v\n", number_of_subpackets)
			bitsProcessed += 11

			for number_of_subpackets > 0 {
				bitsProcessedTemp, tempValue := processPacket(packets[bitsProcessed:])
				queue = append(queue, tempValue)
				number_of_subpackets--
				bitsProcessed += bitsProcessedTemp
			}
		}
	}

	switch packet_type_id {
	case 0:
		// sum packets - their value is the sum of the values of their sub-packets
		total := 0
		for _, v := range queue {
			total += v
		}
		return bitsProcessed, total
	case 1:
		// product packets - their value is the result of multiplying together the values of their sub-packets
		total := 1
		for _, v := range queue {
			total *= v
		}
		return bitsProcessed, total
	case 2:
		// minimum packets - their value is the minimum of the values of their sub-packets
		sort.Ints(queue)
		total := queue[0]
		return bitsProcessed, total
	case 3:
		// maximum packets - their value is the maximum of the values of their sub-packetsimum
		sort.Ints(queue)
		total := queue[len(queue)-1]
		return bitsProcessed, total
	case 5:
		// greater than packets - their value is 1 if the value of the first sub-packet is greater than the value of the second sub-packet; otherwise, their value is 0
		total := 0
		if queue[0] > queue[1] {
			total = 1
		}
		return bitsProcessed, total

	case 6:
		// less than packets - their value is 1 if the value of the first sub-packet is less than the value of the second sub-packet; otherwise, their value is 0
		total := 0
		if queue[0] < queue[1] {
			total = 1
		}
		return bitsProcessed, total
	case 7:
		// equal to packets - their value is 1 if the value of the first sub-packet is equal to the value of the second sub-packet; otherwise, their value is 0
		total := 0
		if queue[0] == queue[1] {
			total = 1
		}
		return bitsProcessed, total
	}

	return 0, 0
}
