<?php

declare(strict_types=1);

namespace Accounts;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\ModelNotFoundException;
use Illuminate\Database\MultipleRecordsFoundException;
use Illuminate\Database\RecordsNotFoundException;

// Accounts with a readonly class that holds a model, and Laravel's exceptions caught by
// their parents and thrown on, written by hand for the harness.
final class Account extends Model
{
    protected $fillable = ['owner', 'email', 'balance'];

    public $timestamps = false;
}

final readonly class Statement
{
    public function __construct(public Account $account, public string $period) {}

    public function heading(): string
    {
        $owner = $this->account->owner;
        return 'Statement for ' . $owner . ' (' . $this->period . ')';
    }

    public function email(): ?string
    {
        return $this->account->email;
    }
}

function open(string $owner, ?string $email, int $balance): int
{
    $account = Account::create(['owner' => $owner, 'email' => $email, 'balance' => $balance]);
    return $account->id;
}

function heading(int $id, string $period): string
{
    $statement = new Statement(Account::findOrFail($id), $period);
    return $statement->heading();
}

function emailOf(int $id): ?string
{
    $statement = new Statement(Account::findOrFail($id), 'now');
    return $statement->email();
}

function byOwner(string $owner): string
{
    try {
        $account = Account::query()->where('owner', $owner)->sole();
        return $account->owner;
    } catch (RecordsNotFoundException $e) {
        return 'none';
    } catch (MultipleRecordsFoundException $e) {
        return 'several';
    }
}

function byRuntime(string $owner): string
{
    try {
        return Account::query()->where('owner', $owner)->sole()->owner;
    } catch (\RuntimeException $e) {
        return $e->getMessage();
    }
}

function either(int $id): string
{
    try {
        return Account::findOrFail($id)->owner;
    } catch (ModelNotFoundException | MultipleRecordsFoundException $e) {
        return 'failed';
    }
}

// A catch of the doubled row leaves a missing one to the caller, which catches it.
function onlyDoubled(string $owner): string
{
    try {
        return Account::query()->where('owner', $owner)->firstOrFail()->owner;
    } catch (MultipleRecordsFoundException $e) {
        return 'doubled';
    }
}

function outer(string $owner): string
{
    try {
        return onlyDoubled($owner);
    } catch (ModelNotFoundException $e) {
        return 'outer: ' . $e->getMessage();
    }
}

function balanceOf(int $id): int
{
    $account = Account::find($id);
    if ($account === null) {
        return -1;
    }
    return $account->balance;
}

function richest(int $floor): string
{
    $found = 'nobody';
    foreach (Account::all() as $account) {
        if ($account->balance >= $floor) {
            $found = $account->owner;
        }
    }
    return $found;
}
