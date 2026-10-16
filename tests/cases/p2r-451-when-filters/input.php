<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\MultipleRecordsFoundException;

final class Owner extends Model
{
    protected $fillable = ['label'];

    public $timestamps = false;
}

final class Code extends Model
{
    protected $fillable = ['name', 'rank', 'owner_id', 'active'];

    protected $casts = ['active' => 'boolean'];

    public $timestamps = false;
}

// when() links of each value type and callback form, with a where between them.
/**
 * @return Collection<int, Code>
 */
function filtered(?string $name, ?int $rank, ?Owner $owner): Collection
{
    return Code::query()
        ->when($name, static function (Builder $query, string $name): void {
            $query->where('name', $name);
        })
        ->where('active', true)
        ->when($rank, function (Builder $query, $rank): void {
            $query->where('rank', $rank);
        })
        ->when($owner, fn (Builder $query, Owner $owner) => $query->where('owner_id', $owner->id))
        ->get();
}

/**
 * @return Collection<int, Code>
 */
function flagged(bool $only): Collection
{
    return Code::query()
        ->when($only, function (Builder $query): void {
            $query->where('active', false);
        })
        ->get();
}

function firstNamed(string $name): string
{
    $code = Code::query()
        ->when($name, fn (Builder $query, string $name) => $query->where('name', $name)->where('rank', 2))
        ->first();
    return $code === null ? '-' : $code->name;
}

function soleOf(int $rank): string
{
    try {
        $code = Code::query()->when($rank, fn (Builder $query, int $rank) => $query->where('rank', $rank))->sole();
        return $code->name;
    } catch (MultipleRecordsFoundException $e) {
        return $e->getMessage();
    }
}

// A second parameter named like the closure's row, and one named like a Rust keyword.
/**
 * @return Collection<int, Code>
 */
function byNames(?Owner $owner, ?string $type): Collection
{
    return Code::query()
        ->when($owner, fn (Builder $query, Owner $row) => $query->where('owner_id', $row->id))
        ->when($type, static function (Builder $query, $type): void {
            $query->where('name', $type);
        })
        ->get();
}

/**
 * @param Collection<int, Code> $codes
 */
function names(Collection $codes): string
{
    $out = '';
    foreach ($codes as $code) {
        $out .= $code->name;
    }
    return $out === '' ? '-' : $out;
}

$first = Owner::create(['label' => 'x']);
$second = Owner::create(['label' => 'y']);
Code::create(['name' => 'a', 'rank' => 1, 'owner_id' => $first->id, 'active' => true]);
Code::create(['name' => 'b', 'rank' => 2, 'owner_id' => $second->id, 'active' => true]);
Code::create(['name' => '0', 'rank' => 2, 'owner_id' => $first->id, 'active' => false]);
Code::create(['name' => 'c', 'rank' => 0, 'owner_id' => $second->id, 'active' => true]);
echo names(filtered(null, null, null)), ' ', names(filtered('a', null, null)), ' ', names(filtered('', 2, null)), "\n";
echo names(filtered('0', 0, $second)), ' ', names(filtered(null, 2, $first)), ' ', names(filtered('b', 2, $second)), "\n";
echo names(flagged(true)), ' ', names(flagged(false)), "\n";
echo firstNamed('b'), ' ', firstNamed('0'), ' ', firstNamed(''), ' ', firstNamed('a'), "\n";
echo soleOf(1), ' ', soleOf(2), ' ', soleOf(0), "\n";
echo names(byNames($first, null)), ' ', names(byNames(null, '0')), ' ', names(byNames($second, 'c')), "\n";
