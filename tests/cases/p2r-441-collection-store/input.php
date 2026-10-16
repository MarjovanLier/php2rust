<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Collection;
use Illuminate\Database\Eloquent\Model;

final class Code extends Model
{
    protected $fillable = ['name', 'kind', 'rank'];

    public $timestamps = false;
}

// get() after where calls, and after none, and all(), each in creation order.
/**
 * @return Collection<int, Code>
 */
function ofKind(string $kind): Collection
{
    return Code::query()->where('kind', $kind)->get();
}

/**
 * @return Collection<int, Code>
 */
function ranked(string $kind, int $rank): Collection
{
    return Code::query()->where('kind', $kind)->where('rank', $rank)->get();
}

/**
 * @return Collection<int, Code>
 */
function unranked(): Collection
{
    return Code::query()->where('rank', null)->get();
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
    return $out . '/' . count($codes);
}

Code::create(['name' => 'a', 'kind' => 'x', 'rank' => 1]);
Code::create(['name' => 'b', 'kind' => 'y', 'rank' => null]);
Code::create(['name' => 'c', 'kind' => 'x', 'rank' => null]);
Code::create(['name' => 'd', 'kind' => 'x', 'rank' => 1]);
echo names(ofKind('x')), ' ', names(ofKind('none')), "\n";
echo names(ranked('x', 1)), ' ', names(unranked()), "\n";
echo names(Code::all()), ' ', names(Code::query()->get()), "\n";
