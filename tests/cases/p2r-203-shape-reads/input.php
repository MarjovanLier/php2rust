<?php

declare(strict_types=1);

final readonly class Person
{
    public function __construct(
        public string $name,
        public int $age,
    ) {}

    /**
     * @param array{name: ?string, age: ?int, when: ?array{day: string}} $data
     */
    public static function fromArray(array $data): self
    {
        return new self(
            name: $data['name'] ?? 'unknown',
            age: $data['age'] ?? 0,
        );
    }
}

/**
 * @param array{name: ?string, age: ?int, when: ?array{day: string}} $data
 */
function day(array $data): string
{
    $when = $data['when'] ?? ['day' => '01'];
    return $when['day'];
}

/**
 * @param array{name: ?string, age: ?int, when: ?array{day: string}} $data
 */
function aged(array $data): string
{
    if ($data['age'] !== null) {
        return $data['name'] ?? 'someone';
    }
    return 'ageless';
}

/**
 * @param array{name: ?string, age: ?int, when: ?array{day: string}} $data
 */
function unaged(array $data): string
{
    if ($data['age'] === null) {
        return $data['name'] ?? 'nobody';
    }
    return 'aged';
}

/**
 * @param array{name: ?string, age: ?int, when: ?array{day: string}} $data
 */
function otherwise(array $data): string
{
    if ($data['age'] !== null) {
        return 'aged';
    } else {
        return $data['name'] ?? 'nobody';
    }
}

/**
 * @param array{inner: array{n: int}, list: list<int>} $data
 */
function nested(array $data): int
{
    return $data['inner']['n'] + count($data['list']);
}

$a = Person::fromArray(['name' => 'Ann', 'age' => 41, 'when' => ['day' => '14']]);
$b = Person::fromArray(['name' => null, 'age' => null, 'when' => null]);
echo $a->name, ' ', $a->age, ' ', $b->name, ' ', $b->age, "\n";
echo day(['when' => ['day' => '14'], 'name' => 'Bo', 'age' => 2]), ' ';
echo day(['name' => null, 'age' => null, 'when' => null]), "\n";
echo nested(['inner' => ['n' => 5], 'list' => [1, 2, 3]]), "\n";
echo aged(['name' => 'Cy', 'age' => 3, 'when' => null]), ' ';
echo aged(['name' => 'Di', 'age' => null, 'when' => null]), "\n";
echo unaged(['name' => 'Ed', 'age' => null, 'when' => null]), ' ';
echo otherwise(['name' => null, 'age' => null, 'when' => null]), "\n";
