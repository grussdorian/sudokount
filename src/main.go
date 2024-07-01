package main

import (
	"bufio"
	"fmt"
	"math/bits"
	"os"
	"runtime"
	"strconv"
	"time"
)

const (
	maxBDim     = 8
	cellValSize = 1
)

type cellVal struct {
	v [cellValSize]uint64
}

type cellCoord struct {
	r, c int
}

type sudoku struct {
	bdim      int
	dim       int
	peersSize int
	grid      []int
	unitList  [][][][]cellCoord
	peers     [][][]cellCoord
	values    [][]cellVal
	solCount  uint64
}

func newSudoku(bdim int, grid []int) *sudoku {
	dim := bdim * bdim
	s := &sudoku{
		bdim:      bdim,
		dim:       dim,
		peersSize: 3*dim - 2*bdim - 1,
		grid:      grid,
		unitList:  make([][][][]cellCoord, dim),
		peers:     make([][][]cellCoord, dim),
		values:    make([][]cellVal, dim),
	}

	for i := 0; i < dim; i++ {
		s.unitList[i] = make([][][]cellCoord, dim)
		s.peers[i] = make([][]cellCoord, dim)
		s.values[i] = make([]cellVal, dim)
		for j := 0; j < dim; j++ {
			s.unitList[i][j] = make([][]cellCoord, 3)
			s.peers[i][j] = make([]cellCoord, s.peersSize)
			for k := 0; k < 3; k++ {
				s.unitList[i][j][k] = make([]cellCoord, dim)
			}
		}
	}

	s.init()
	if !s.parseGrid() {
		return nil
	}

	return s
}

func (s *sudoku) init() {
	for i := 0; i < s.dim; i++ {
		ibase := i / s.bdim * s.bdim
		for j := 0; j < s.dim; j++ {
			for pos := 0; pos < s.dim; pos++ {
				s.unitList[i][j][0][pos] = cellCoord{i, pos} // row
				s.unitList[i][j][1][pos] = cellCoord{pos, j} // column
			}
			jbase := j / s.bdim * s.bdim
			pos := 0
			for k := 0; k < s.bdim; k++ {
				for l := 0; l < s.bdim; l++ {
					s.unitList[i][j][2][pos] = cellCoord{ibase + k, jbase + l} // box
					pos++
				}
			}
		}
	}

	for i := 0; i < s.dim; i++ {
		for j := 0; j < s.dim; j++ {
			pos := 0
			for k := 0; k < s.dim; k++ {
				if s.unitList[i][j][0][k].c != j {
					s.peers[i][j][pos] = s.unitList[i][j][0][k]
					pos++
				}
			}
			for k := 0; k < s.dim; k++ {
				sq := s.unitList[i][j][1][k]
				if sq.r != i {
					s.peers[i][j][pos] = sq
					pos++
				}
				sq = s.unitList[i][j][2][k]
				if sq.r != i && sq.c != j {
					s.peers[i][j][pos] = sq
					pos++
				}
			}
		}
	}
}

func (s *sudoku) parseGrid() bool {
	for i := 0; i < s.dim; i++ {
		for j := 0; j < s.dim; j++ {
			for k := 1; k <= s.dim; k++ {
				s.values[i][j].set(k)
			}
		}
	}

	for i := 0; i < s.dim; i++ {
		for j := 0; j < s.dim; j++ {
			if s.grid[i*s.dim+j] > 0 && !s.assign(i, j, s.grid[i*s.dim+j]) {
				return false
			}
		}
	}

	return true
}

func (cv *cellVal) get(p int) bool {
	return cv.v[(p-1)/64]&(1<<uint((p-1)%64)) != 0
}

func (cv *cellVal) set(p int) {
	cv.v[(p-1)/64] |= 1 << uint((p-1)%64)
}

func (cv *cellVal) unset(p int) {
	cv.v[(p-1)/64] &= ^(1 << uint((p-1)%64))
}

func (cv *cellVal) count() int {
	count := 0
	for i := 0; i < cellValSize; i++ {
		count += bits.OnesCount64(cv.v[i])
	}
	return count
}

func (cv *cellVal) digit() int {
	count := cv.count()
	if count != 1 {
		return -1
	}
	for i := 0; i < cellValSize; i++ {
		if cv.v[i] != 0 {
			return i*64 + bits.TrailingZeros64(cv.v[i]) + 1
		}
	}
	return -1
}

func (s *sudoku) assign(i, j, d int) bool {
	for d2 := 1; d2 <= s.dim; d2++ {
		if d2 != d {
			if !s.eliminate(i, j, d2) {
				return false
			}
		}
	}
	return true
}

func (s *sudoku) eliminate(i, j, d int) bool {
	if !s.values[i][j].get(d) {
		return true
	}

	s.values[i][j].unset(d)

	count := s.values[i][j].count()
	if count == 0 {
		return false
	} else if count == 1 {
		for k := 0; k < s.peersSize; k++ {
			if !s.eliminate(s.peers[i][j][k].r, s.peers[i][j][k].c, s.values[i][j].digit()) {
				return false
			}
		}
	}

	for k := 0; k < 3; k++ {
		cont := 0
		pos := 0
		for ii := 0; ii < s.dim; ii++ {
			if s.values[s.unitList[i][j][k][ii].r][s.unitList[i][j][k][ii].c].get(d) {
				cont++
				pos = ii
			}
		}
		if cont == 0 {
			return false
		} else if cont == 1 {
			if !s.assign(s.unitList[i][j][k][pos].r, s.unitList[i][j][k][pos].c, d) {
				return false
			}
		}
	}
	return true
}

func (s *sudoku) search(ch chan<- uint64) {
	solved := true
	for i := 0; i < s.dim && solved; i++ {
		for j := 0; j < s.dim; j++ {
			if s.values[i][j].count() != 1 {
				solved = false
				break
			}
		}
	}
	if solved {
		ch <- 1
		return
	}

	min := s.dim + 1
	minI, minJ := -1, -1
	for i := 0; i < s.dim; i++ {
		for j := 0; j < s.dim; j++ {
			used := s.values[i][j].count()
			if used > 1 && used < min {
				min = used
				minI, minJ = i, j
			}
		}
	}

	localCh := make(chan uint64)
	activeSearches := 0

	for k := 1; k <= s.dim; k++ {
		if s.values[minI][minJ].get(k) {
			localS := s.clone()
			if localS.assign(minI, minJ, k) {
				activeSearches++
				go localS.search(localCh)
			}
		}
	}

	var count uint64
	for i := 0; i < activeSearches; i++ {
		count += <-localCh
	}
	ch <- count
}

func (s *sudoku) clone() *sudoku {
	newS := &sudoku{
		bdim:      s.bdim,
		dim:       s.dim,
		peersSize: s.peersSize,
		grid:      make([]int, len(s.grid)),
		unitList:  s.unitList,
		peers:     s.peers,
		values:    make([][]cellVal, s.dim),
	}
	copy(newS.grid, s.grid)
	for i := 0; i < s.dim; i++ {
		newS.values[i] = make([]cellVal, s.dim)
		copy(newS.values[i], s.values[i])
	}
	return newS
}

func (s *sudoku) solve() {
	ch := make(chan uint64)
	go s.search(ch)
	s.solCount = <-ch
}

func main() {
	// Set the number of OS threads to use
	numCPU := runtime.NumCPU()
	// numCPU := 20
	runtime.GOMAXPROCS(numCPU)
	fmt.Printf("Running with %d threads\n", numCPU)

	scanner := bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)

	// Read the size of the puzzle
	if !scanner.Scan() {
		fmt.Println("Failed to read size")
		os.Exit(1)
	}
	size, err := strconv.Atoi(scanner.Text())
	if err != nil {
		fmt.Println("Failed to parse size:", err)
		os.Exit(1)
	}

	if size > maxBDim {
		fmt.Printf("Size exceeds maximum allowed (%d)\n", maxBDim)
		os.Exit(1)
	}

	bufSize := size * size * size * size
	buf := make([]int, 0, bufSize)

	// Read the rest of the input
	for len(buf) < bufSize {
		if !scanner.Scan() {
			fmt.Println("Failed to read input")
			os.Exit(1)
		}
		num, err := strconv.Atoi(scanner.Text())
		if err != nil {
			fmt.Println("Failed to parse number:", err)
			os.Exit(1)
		}
		buf = append(buf, num)
	}

	if len(buf) != bufSize {
		fmt.Println("Incorrect number of input values")
		os.Exit(1)
	}

	s := newSudoku(size, buf)
	if s != nil {
		start := time.Now()
		s.solve()
		duration := time.Since(start)

		if s.solCount > 0 {
			fmt.Printf("Number of solutions: %d\n", s.solCount)
			fmt.Printf("Time taken: %v\n", duration)
		} else {
			fmt.Println("Could not solve puzzle.")
		}
	} else {
		fmt.Println("Could not load puzzle.")
	}
}
