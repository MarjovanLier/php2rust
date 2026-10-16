<?php

declare(strict_types=1);

namespace App\Exceptions;

// An exception class with nothing in it joins the class tree, also under another of them.
class RoleDoesNotExistException extends \Exception {}

final class MissingRoleException extends RoleDoesNotExistException {}

// A constructor that builds the message becomes a function (P2R-391).
class FieldLengthInvalidException extends \InvalidArgumentException
{
    public function __construct(
        public readonly string $fieldName,
        public readonly int $actualLength,
        public readonly ?int $minLength = null,
        public readonly ?int $maxLength = null,
    ) {
        if ($minLength !== null && $maxLength !== null) {
            $firstPart = sprintf('Field %s must be between %d and %d characters in length', $fieldName, $minLength, $maxLength);
        } elseif ($maxLength !== null) {
            $firstPart = sprintf('Field %s must be at most %d characters in length', $fieldName, $maxLength);
        } elseif ($minLength !== null) {
            $firstPart = sprintf('Field %s must be at least %d characters in length', $fieldName, $minLength);
        } else {
            $firstPart = sprintf('Field %s length is invalid', $fieldName);
        }

        $message = sprintf('%s. Length given is %d', $firstPart, $actualLength);

        parent::__construct($message);
    }
}

class RoleAlreadyExistsException extends \Exception
{
    public function __construct(string $message = 'Role already exists')
    {
        parent::__construct($message, 409);
    }
}

function check(string $name): string
{
    if (strlen($name) < 3) {
        throw new FieldLengthInvalidException($name, strlen($name), 3, 10);
    }
    if ($name === 'admin') {
        throw new RoleAlreadyExistsException();
    }
    if ($name === 'ghost') {
        throw new MissingRoleException('no ghost', 404);
    }
    return $name;
}

foreach (['ab', 'admin', 'ghost', 'editor'] as $name) {
    try {
        echo check($name), "\n";
    } catch (RoleDoesNotExistException $e) {
        echo 'missing: ', $e->getMessage(), ' ', $e->getCode(), "\n";
    } catch (\InvalidArgumentException $e) {
        echo 'invalid: ', $e->getMessage(), "\n";
    } catch (\Exception $e) {
        echo 'other: ', $e->getMessage(), ' ', $e->getCode(), "\n";
    }
}
