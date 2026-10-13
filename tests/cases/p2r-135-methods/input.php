<?php

declare(strict_types=1);

readonly class Counter
{
    public function __construct(public int $start)
    {
    }

    public function down(int $n): int
    {
        if ($n <= 0) {
            return $this->start;
        }
        return $this->down($n - 1) + 1;
    }

    public function report(): void
    {
        echo 'start ', $this->start, "\n";
    }

    public function twice(): int
    {
        return $this->down(1) + $this->down(2);
    }

    public function orStart(?int $n): int
    {
        return $n ?? $this->orStart($this->start);
    }
}

$c = new Counter(10);
$c->report();
echo $c->down(3), ' ', $c->twice(), ' ', $c->orStart(null), "\n";
for ($i = 0; $i < 2; $c->report()) {
    $i++;
}
