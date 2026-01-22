# entrega-Solana

Ejercitación 6 **Solana**
Desarrollo de Software para Blockchain - Cursada 2025

# Sistema de Votación con Propuestas

El objetivo de esta ejercitación es desarrollar un contrato inteligente en Solana que permita gestionar propuestas y registrar votos de los usuarios de manera segura, transparente y verificable on-chain.

## Requisitos del Sistema

- Cada propuesta será creada por el administrador del contrato (owner), que será la cuenta que despliega inicialmente el contrato.
- Los participantes podrán emitir un único voto por propuesta.
- El contrato deberá llevar registro de todas las propuestas generadas, la cantidad de votos que obtiene cada una, y qué cuentas ya han votado.

# Garantías del Sistema

El sistema deberá garantizar que:

- No se puedan votar propuestas inexistentes.
- Ningún usuario pueda votar dos veces la misma propuesta.
- Solo el owner pueda crear nuevas propuestas.
- Toda la información relevante quede almacenada on-chain.
- El contrato deberá exponer de forma clara y pública los datos de cada propuesta y la cantidad total de propuestas.

# Modelo de funcionamiento

Todo el estado se almacena en cuentas, siguiendo este esquema:

- Config: guarda el owner y la cantidad de propuestas
- Proposal: representa una propuesta individual
- Vote: representa el voto de un usuario sobre una propuesta

Cada una de estas entidades se modela como una PDA (Program Derived Address).

# Tests

Los tests están implementados en TypeScript usando Mocha

```bash
npx mocha -r ts-node/register tests/voting.ts

```

## Importante: los tests

- No ejecutan el programa en la blockchain
- No requieren Solana CLI
- Verifican la lógica del sistema
- Validan reglas como:
  - creación de propuestas
  - voto único
  - control de flujo

Esto se debe a limitaciones del entorno de ejecución, pero el diseño del contrato sigue el modelo real de Solana.

# Sobre Anchor

Si bien el proyecto no utiliza directamente el framework Anchor, el diseño del contrato sigue el mismo modelo conceptual que Anchor propone.

En particular:

- No se utilizan estructuras de estado globales.
- El estado se representa mediante cuentas independientes.
- Las propuestas y los votos se modelan como entidades separadas.
- El acceso y la unicidad se controlan de forma determinística.
- El diseño es compatible con el uso de PDAs.
