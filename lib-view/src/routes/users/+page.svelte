<script lang="ts">
	import { _ } from "svelte-i18n";
	import Container from "../../components/basic/Container.svelte";
	import UserList from "./UserList.svelte";
	import UserSearch, { UserParams } from "./UserSearch.svelte";
	import type { User } from "./UserView.svelte";
	import UserView from "./UserView.svelte";

	let params: UserParams;
	let active: User | null;
	let isNew: boolean;
</script>

<svelte:head>
	<title>{$_(".user")}</title>
	<meta name="description" content={$_(".user")} />
</svelte:head>

<Container>
	<span slot="list">
		<UserSearch bind:params />
		<UserList
			bind:active
			bind:isNew
			items={[
				{
					account: "nils.wrenger",
					forename: "Nils",
					surname: "Wrenger",
					role: "Jahrgang 10",
					permission: true
				},
				{
					account: "lars.wrenger",
					forename: "Lars",
					surname: "Wrenger",
					role: "-",
					permission: true
				},
				{
					account: "emma.johnson",
					forename: "Emma",
					surname: "Johnson",
					role: "Student",
					permission: true
				},
				{
					account: "alex.smith",
					forename: "Alex",
					surname: "Smith",
					role: "Teacher",
					permission: true
				},
				{
					account: "sophie.miller",
					forename: "Sophie",
					surname: "Miller",
					role: "Administrator",
					permission: true
				},
				{
					account: "james.brown",
					forename: "James",
					surname: "Brown",
					role: "Parent",
					permission: false
				},
				{
					account: "olivia.jones",
					forename: "Olivia",
					surname: "Jones",
					role: "Student",
					permission: true
				},
				{
					account: "david.wilson",
					forename: "David",
					surname: "Wilson",
					role: "Teacher",
					permission: true
				},
				{
					account: "sarah.davis",
					forename: "Sarah",
					surname: "Davis",
					role: "Student",
					permission: true
				},
				{
					account: "michael.white",
					forename: "Michael",
					surname: "White",
					role: "Teacher",
					permission: true
				},
				{
					account: "linda.jackson",
					forename: "Linda",
					surname: "Jackson",
					role: "Teacher",
					permission: true
				},
				{
					account: "peter.johnson",
					forename: "Peter",
					surname: "Johnson",
					role: "Teacher",
					permission: true
				},
				{
					account: "mary.smith",
					forename: "Mary",
					surname: "Smith",
					role: "Administrator",
					permission: true
				},
				{
					account: "robert.brown",
					forename: "Robert",
					surname: "Brown",
					role: "Teacher",
					permission: true
				},
				{
					account: "karen.white",
					forename: "Karen",
					surname: "White",
					role: "Teacher",
					permission: true
				},
				{
					account: "jennifer.davis",
					forename: "Jennifer",
					surname: "Davis",
					role: "Teacher",
					permission: true
				},
				{
					account: "mark.johnson",
					forename: "Mark",
					surname: "Johnson",
					role: "Teacher",
					permission: true
				},
				{
					account: "william.brown",
					forename: "William",
					surname: "Brown",
					role: "Parent",
					permission: false
				},
				{
					account: "karen.jones",
					forename: "Karen",
					surname: "Jones",
					role: "Administrator",
					permission: true
				},
				{
					account: "emily.davis",
					forename: "Emily",
					surname: "Davis",
					role: "Student",
					permission: true
				}
			]}
		/>
	</span>
	<UserView slot="view" bind:user={active} bind:isNew />
</Container>
