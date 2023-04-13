package essentials_test

import (
	"crypto/sha256"
	"fmt"
	"testing"
)

// Merkle tree struct
type MerkleTree struct {
	RootNode *MerkleNode
}

type MerkleNode struct {
	Left  *MerkleNode
	Right *MerkleNode
	Data  []byte
}

func TestMerkleTree(t *testing.T) {
	data := [][]byte{
		[]byte("my"),
		[]byte("milkshake"),
		[]byte("brings all boys"),
		[]byte("to the yard"),
	}

	tree := NewMerkleTree(data)

	fmt.Printf("Root hash: %x\n", tree.RootNode.Data)
}

func NewMerkleNode(left, right *MerkleNode, data []byte) *MerkleNode {
	node := MerkleNode{}

	if left == nil && right == nil {
		// if it is a leaf, the hash is data itself
		hash := sha256.Sum256(data)
		node.Data = hash[:]
	} else {
		// if it is a node, the hash is
		hash := sha256.Sum256(append(left.Data, right.Data...))
		node.Data = hash[:]
	}

	node.Left = left
	node.Right = right

	return &node
}

func NewMerkleTree(data [][]byte) *MerkleTree {
	var nodes []MerkleNode

	if len(data)%2 != 0 {
		data = append(data, data[len(data)-1])
	}

	for _, datum := range data {
		node := NewMerkleNode(nil, nil, datum)
		nodes = append(nodes, *node)
	}

	for len(nodes) > 1 {
		if len(nodes)%2 != 0 {
			nodes = append(nodes, nodes[len(nodes)-1])
		}

		var level []MerkleNode

		for i := 0; i < len(nodes); i += 2 {
			node := NewMerkleNode(&nodes[i], &nodes[i+1], nil)
			level = append(level, *node)
		}

		nodes = level
	}

	// create a tree by simply assigning the root node
	tree := MerkleTree{&nodes[0]}

	return &tree
}
