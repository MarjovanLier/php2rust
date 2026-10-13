<?php

declare(strict_types=1);

final class Node
{
    public int $n = 0;
    public ?int $m = null;
    /** @var list<int> */
    public array $xs = [];
    /** @var list<float> */
    public array $fs = [];
}

function make(): Node
{
    return new Node();
}

function throughCall(): void
{
    make()->n = 1;
}

function coalesce(Node $a): void
{
    $a->m ??= 1;
}

function element(Node $a): void
{
    $a->xs[0] = 1;
}

function asValue(Node $a): int
{
    return $a->n = 2;
}

function intIntoFloats(Node $a): void
{
    $a->fs[] = 1;
}

/**
 * @param list<Node> $nodes
 */
function replace(array $nodes): void
{
    foreach ($nodes as $node) {
        echo $node->n, "\n";
        $node = new Node();
        echo $node->n, "\n";
    }
}
