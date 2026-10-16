<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\ModelNotFoundException;
use Illuminate\Database\MultipleRecordsFoundException;
use Illuminate\Database\RecordsNotFoundException;

final class Tag extends Model
{
    protected $fillable = ['label'];

    public $timestamps = false;
}

// RecordsNotFoundException is the parent of ModelNotFoundException in Laravel's class tree.
function byParent(string $label): string
{
    try {
        $tag = Tag::query()->where('label', $label)->sole();
        return $tag->label;
    } catch (RecordsNotFoundException $e) {
        return 'none';
    }
}

// Both classes extend PHP's RuntimeException.
function byRuntime(string $label): string
{
    try {
        $tag = Tag::query()->where('label', $label)->sole();
        return $tag->label;
    } catch (\RuntimeException $e) {
        return $e->getMessage();
    }
}

function either(int $id): string
{
    try {
        $tag = Tag::findOrFail($id);
        return $tag->label;
    } catch (ModelNotFoundException | MultipleRecordsFoundException $e) {
        return 'failed';
    }
}

// A catch of the doubled row leaves a missing one to the caller, which throws it on.
function onlyDoubled(string $label): string
{
    try {
        $tag = Tag::query()->where('label', $label)->firstOrFail();
        return $tag->label;
    } catch (MultipleRecordsFoundException $e) {
        return 'doubled';
    }
}

function outer(string $label): string
{
    try {
        return onlyDoubled($label);
    } catch (\Exception $e) {
        return 'outer: ' . $e->getMessage();
    }
}

Tag::create(['label' => 'red']);
Tag::create(['label' => 'blue']);
Tag::create(['label' => 'blue']);
echo byParent('red'), ' ', byParent('green'), "\n";
echo byRuntime('blue'), "\n";
echo either(2), ' ', either(7), "\n";
echo outer('blue'), ' ', outer('green'), "\n";
